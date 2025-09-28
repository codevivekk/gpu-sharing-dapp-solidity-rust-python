use crate::config::YourContractError;
use crate::helper::save_jobs;
use crate::models::{Job, Node};
use crate::state::AppState;
use actix_web;
use actix_web::{web, HttpResponse, Responder};
use ethers::abi::AbiDecode;
use ethers::contract::ContractError;
use ethers::types::{Address, H256};
use ethers_contract::AbiError;
use log::{error, info};
use serde::Deserialize;
use serde_json::json;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::str::FromStr;
use uuid::Uuid;

fn save_nodes(nodes: &Vec<Node>) {
    let file_path = "nodes.json";
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)
        .unwrap();
    let json = serde_json::to_string_pretty(nodes).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

pub async fn register_node(node: web::Json<Node>, data: web::Data<AppState>) -> impl Responder {
    let mut nodes = data.nodes.lock().unwrap();
    let mut new_node = node.into_inner();

    // Generate server-side node ID if not provided
    if new_node.node_id.is_empty() {
        new_node.node_id = Uuid::new_v4().to_string();
    }

    new_node.status = "idle".to_string();
    new_node.active = true;

    nodes.push(new_node.clone());

    save_nodes(&nodes);

    HttpResponse::Ok().json(new_node)
}

pub async fn get_all_nodes(data: web::Data<AppState>) -> impl Responder {
    let nodes = data.nodes.lock().unwrap();
    HttpResponse::Ok().json(&*nodes)
}

pub async fn get_node_jobs(
    node_id: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    let node_id = node_id.into_inner();
    println!("Received request for node_id: {}", node_id);

    let file_content = match fs::read_to_string("nodes.json") {
        Ok(content) => {
            println!("Successfully read nodes.json");
            content
        }
        Err(err) => {
            error!("Failed to read nodes.json: {}", err);
            return HttpResponse::InternalServerError()
                .json(json!({ "error": "Failed to read nodes file" }));
        }
    };
    let nodes: Vec<Node> = match serde_json::from_str::<Vec<Node>>(&file_content) {
        Ok(nodes) => {
            println!("Parsed {} nodes from nodes.json", nodes.len());
            nodes
        }
        Err(err) => {
            error!("Failed to parse nodes.json: {}", err);
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": "Failed to parse nodes file"}));
        }
    };

    let node = nodes.iter().find(|n| n.node_id == node_id);
    if node.is_none() {
        println!("Node not found or inactive: {}", node_id);
        return HttpResponse::NotFound().json(json!({ "error": "Node not found or inactive" }));
    }
    let node = node.unwrap();
    println!(
        "Found active node: {} (specs: {})",
        node.node_id, node.gpu_specs
    );

    let jobs = data.jobs.lock().unwrap();
    println!("Loaded {} jobs from in-memory state", jobs.len());

    let assigned_jobs: Vec<Job> = jobs
        .iter()
        .filter(|j| j.required_specs == node.gpu_specs)
        .cloned()
        .collect();

    println!(
        "Returning {} assigned jobs for node {}",
        assigned_jobs.len(),
        node.node_id
    );

    HttpResponse::Ok().json(assigned_jobs)
}

fn select_node_for_job(job: &Job, nodes: &Vec<Node>) -> Option<Node> {
    nodes
        .iter()
        .find(|n| n.active && n.status == "idle" && n.gpu_specs == job.required_specs)
        .cloned()
}

#[derive(Deserialize)]
pub struct AssignRequest {
    pub job_id: String,
    pub address: Option<String>, 
}

pub async fn assign_provider(
    req: web::Json<AssignRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    info!(
        "Received request to assign provider for job_id: {}",
        req.job_id
    );

    let wallet_address = match validate_wallet(&req) {
        Ok(addr) => addr,
        Err(resp) => return resp,
    };

    let job_id = req.job_id.clone();

    let selected_node_opt = match find_and_select_node(&data, &job_id) {
        Ok(opt) => opt,
        Err(resp) => return resp,
    };

    if let Some(selected_node) = selected_node_opt {
        if let Err(resp) = assign_on_chain(&data, &job_id, wallet_address).await {
            return resp;
        }

        let response_payload =
            update_job_and_node_state(&data, &job_id, wallet_address, &selected_node);
        HttpResponse::Ok().json(response_payload)
    } else {
        println!("No eligible nodes found for job_id: {}", job_id);
        HttpResponse::ServiceUnavailable().json("No eligible nodes available")
    }
}

fn validate_wallet(req: &AssignRequest) -> Result<Address, HttpResponse> {
    let address_str = match req.address.as_ref().filter(|s| !s.is_empty()) {
        Some(s) => s,
        None => {
            error!("Wallet address missing");
            return Err(HttpResponse::BadRequest().json("Wallet address required"));
        }
    };

    match Address::from_str(address_str) {
        Ok(addr) => Ok(addr),
        Err(_) => {
            error!("Invalid wallet address: {}", address_str);
            Err(HttpResponse::BadRequest().json("Invalid wallet address"))
        }
    }
}

fn find_and_select_node(
    data: &web::Data<AppState>,
    job_id: &str,
) -> Result<Option<crate::nodes::Node>, HttpResponse> {
    let mut jobs = data.jobs.lock().unwrap();
    let nodes = data.nodes.lock().unwrap();

    let job_opt = jobs.iter_mut().find(|j| j.job_id == job_id);

    if job_opt.is_none() {
        println!("Job not found: {}", job_id);
        return Err(HttpResponse::NotFound().json("Job not found"));
    }

    let job = job_opt.unwrap();
    if job.status != "pending" {
        println!("Job {} not pending", job_id);
        return Err(HttpResponse::BadRequest().json("Job not pending"));
    }

    println!("Selecting node for job_id: {}", job_id);
    Ok(select_node_for_job(job, &nodes))
}

async fn assign_on_chain(
    data: &web::Data<AppState>,
    job_id: &str,
    wallet_address: Address,
) -> Result<(), HttpResponse> {
    let contract = &data.cfg.contract;
    match contract.method::<_, ()>("assignProvider", (job_id.to_string(), wallet_address)) {
        Ok(call) => {
            println!(
                "Calling blockchain contract assignProvider for job_id: {}",
                job_id
            );
            match call.send().await {
                Ok(tx) => {
                    if tx.await.is_err() {
                        println!("Transaction failed to confirm for job_id: {}", job_id);
                        return Err(HttpResponse::InternalServerError()
                            .json("Transaction failed to confirm"));
                    }
                    println!("Transaction confirmed for job_id: {}", job_id);
                    Ok(())
                }
                Err(e) => {
                    println!("Blockchain assignment failed: {:?}", e);
                    Err(HttpResponse::InternalServerError().json("Blockchain assignment failed"))
                }
            }
        }
        Err(e) => {
            println!("Contract call preparation failed: {:?}", e);
            Err(HttpResponse::InternalServerError().json("Contract call preparation failed"))
        }
    }
}

fn update_job_and_node_state(
    data: &web::Data<AppState>,
    job_id: &str,
    wallet_address: Address,
    selected_node: &crate::nodes::Node,
) -> serde_json::Value {
    println!("[LOG] Acquiring jobs lock");
    let mut jobs = data.jobs.lock().unwrap();
    println!("[LOG] Acquiring nodes lock");
    let mut nodes = data.nodes.lock().unwrap();

    println!("[LOG] Searching for job with id: {}", job_id);
    let job = jobs.iter_mut().find(|j| j.job_id == job_id).unwrap();
    println!("[LOG] Updating job provider_address, assigned_node, status");
    job.provider_address = Some(wallet_address.to_string());
    job.assigned_node = Some(selected_node.node_id.clone());
    job.status = "assigned".to_string();

    println!(
        "[LOG] Searching for node with id: {} and setting status to 'busy'",
        selected_node.node_id
    );
    if let Some(n) = nodes
        .iter_mut()
        .find(|n| n.node_id == selected_node.node_id)
    {
        n.status = "busy".to_string();
    } else {
        println!(
            "[WARN] Selected node {} not found in nodes list",
            selected_node.node_id
        );
    }

    println!("[LOG] Cloning job and node for response");
    let job_clone = job.clone();
    let selected_node_clone = selected_node.clone();

    println!("[LOG] Dropping jobs and nodes locks");
    drop(jobs);
    drop(nodes);

    println!("[LOG] Reacquiring jobs and nodes locks for saving");
    let jobs = data.jobs.lock().unwrap();
    let nodes = data.nodes.lock().unwrap();
    println!("[LOG] Saving jobs");
    save_jobs(&jobs);
    println!("[LOG] Saving nodes");
    crate::nodes::save_nodes(&nodes);

    println!(
        "[LOG] Job {} assigned to node {} successfully",
        job_clone.job_id, selected_node_clone.node_id
    );

    serde_json::json!({
        "success": true,
        "job": job_clone,
        "node": selected_node_clone
    })
}

#[derive(Deserialize)]
pub struct JobResult {
    pub node_id: String,
    pub result_hash: String,
}

pub async fn submit_job_result(
    job_id: web::Path<String>,
    req: web::Json<JobResult>,
    data: web::Data<AppState>,
) -> impl Responder {
    let node_id = &req.node_id.clone();
    let result_hash = &req.result_hash.clone();
    println!("[INFO] Submit job result called for job_id: {}", *job_id);

    let mut jobs = data.jobs.lock().unwrap();
    let mut nodes = data.nodes.lock().unwrap();

    let job_opt = jobs.iter_mut().find(|j| j.job_id == *job_id);
    if job_opt.is_none() {
        println!("[WARN] Job not found: {}", *job_id);
        return HttpResponse::NotFound().json(json!({"error": "Job not found"}));
    }
    let job = job_opt.unwrap();
    println!("[INFO] Found job {}, status: {}", job.job_id, job.status);

    update_job_state_to_completed(job, &result_hash);

    update_node_status(&mut nodes, node_id);

    let job_clone = job.clone();
    drop(jobs);
    drop(nodes);

    persist_state(&data);

    if let Err(e) = submit_result_to_blockchain(
        &data,
        &job_clone.job_id,
        job_clone.result_hash.as_ref().unwrap(),
    )
    .await
    {
        println!("[ERROR] Blockchain transaction error: {:?}", e);
    }

    HttpResponse::Ok().json(job_clone)
}

fn update_job_state_to_completed(job: &mut Job, result_hash: &str) {
    job.status = "completed".to_string();
    job.result_hash = Some(result_hash.to_owned());
    println!(
        "[INFO] Job marked completed with result_hash: {}",
        result_hash
    );
}

fn update_node_status(nodes: &mut Vec<Node>, node_id: &str) {
    if let Some(node) = nodes.iter_mut().find(|n| n.node_id == node_id) {
        node.status = "idle".to_string();
        println!("[INFO] Node {} marked as idle", node.node_id);
    } else {
        println!("[WARN] Node {} not found while updating status", node_id);
    }
}

fn persist_state(data: &web::Data<AppState>) {
    let jobs = data.jobs.lock().unwrap();
    let nodes = data.nodes.lock().unwrap();
    save_jobs(&jobs);
    crate::nodes::save_nodes(&nodes);
    println!("[INFO] Persisted jobs and nodes to storage");
}

async fn submit_result_to_blockchain(
    data: &web::Data<AppState>,
    job_id: &str,
    result_hash: &str,
) -> Result<(), YourContractError> {
    println!(
        "[INFO] Submitting result to blockchain for job: {}, hash: {}",
        job_id, result_hash
    );
    println!(
        "[INFO] Using contract at address: {:?}",
        data.cfg.contract.address()
    );

    let contract = &data.cfg.contract;
    println!("[INFO] Preparing to convert result_hash to H256");
    let result_hash_bytes = match H256::from_str(result_hash) {
        Ok(val) => {
            println!("[INFO] Converted result_hash to H256: {:?}", val);
            val
        }
        Err(e) => {
            eprintln!("[ERROR] Failed to parse result_hash as H256: {:?}", e);
            return Err(ContractError::AbiError(AbiError::DecodingError(
                ethers::abi::Error::InvalidData,
            )));
        }
    };
    println!("[INFO] Calling contract method submitResult");
    let submit_call =
        match contract.method::<_, ()>("submitResult", (job_id.to_string(), result_hash_bytes)) {
            Ok(call) => {
                println!("[INFO] Prepared contract call for submitResult");
                call
            }
            Err(e) => {
                eprintln!("[ERROR] Failed to prepare contract method call: {:?}", e);
                return Err(ContractError::AbiError(e));
            }
        };
    println!("[INFO] Sending transaction to blockchain");
    let pending_tx = match submit_call.send().await {
        Ok(tx) => {
            println!("[INFO] Transaction sent with hash: {:?}", tx.tx_hash());
            tx
        }
        Err(e) => {
            if let Some(revert_data) = e.as_revert() {
                if let Ok(reason) = String::decode(&revert_data.0[4..]) {
                    eprintln!("[ERROR] Transaction reverted: {}", reason);
                } else {
                    eprintln!("[ERROR] Transaction reverted, but failed to decode reason");
                }
            } else {
                eprintln!("[ERROR] Transaction send failed: {:?}", e);
            }
            return Err(e.into());
        }
    };
    println!("[INFO] Awaiting transaction confirmation");
    let receipt = match pending_tx.await {
        Ok(Some(r)) => {
            println!("[INFO] Transaction confirmed: {:?}", r.transaction_hash);
            r
        }
        Ok(None) => {
            println!("[WARN] Transaction receipt is None");
            return Err(ContractError::ProviderError {
                e: ethers::providers::ProviderError::CustomError(
                    "Transaction receipt was None".into(),
                ),
            });
        }
        Err(e) => {
            eprintln!("[ERROR] Awaiting transaction receipt failed: {:?}", e);
            return Err(e.into());
        }
    };
    println!("[INFO] Result submission transaction completed");
    let owner_contract = &data.cfg.owner_contract;
    println!(
        "[INFO] Preparing to call owner contract release for job_id: {}",
        job_id
    );
    let release_call = match owner_contract.method::<_, ()>("release", job_id.to_string()) {
        Ok(call) => call,
        Err(e) => {
            eprintln!(
                "[ERROR] Failed to prepare owner contract method call: {:?}",
                e
            );
            return Err(ContractError::AbiError(e));
        }
    };
    println!("[INFO] Sending release transaction to blockchain");
    let pending_release_tx = match release_call.send().await {
        Ok(tx) => {
            println!(
                "[INFO] Release transaction sent with hash: {:?}",
                tx.tx_hash()
            );
            tx
        }
        Err(e) => {
            eprintln!("[ERROR] Release transaction send failed: {:?}", e);
            return Err(e.into());
        }
    };
    println!("[INFO] Awaiting release transaction confirmation");
    let release_receipt = match pending_release_tx.await {
        Ok(Some(r)) => {
            println!(
                "[INFO] Release transaction confirmed: {:?}",
                r.transaction_hash
            );
            r
        }
        Ok(None) => {
            println!("[WARN] Release transaction receipt is None");
            return Err(ContractError::ProviderError {
                e: ethers::providers::ProviderError::CustomError(
                    "Transaction receipt was None".into(),
                ),
            });
        }
        Err(e) => {
            eprintln!(
                "[ERROR] Awaiting release transaction receipt failed: {:?}",
                e
            );
            return Err(e.into());
        }
    };

    Ok(())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/nodes")
            .route("", web::get().to(get_all_nodes))
            .route("/register", web::post().to(register_node))
            .route("/{id}/jobs", web::get().to(get_node_jobs))
            .route("/{id}/result", web::post().to(submit_job_result))
            .route("/assign-provider", web::post().to(assign_provider)),
    );
}
