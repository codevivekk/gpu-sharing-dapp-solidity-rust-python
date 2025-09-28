use crate::config::AppConfig;
use crate::models::{Job, Node};
use std::fs;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub jobs: Arc<Mutex<Vec<Job>>>,
    pub nodes: Arc<Mutex<Vec<Node>>>,
    pub cfg: Arc<AppConfig>,
}

impl AppState {
    pub async fn new() -> anyhow::Result<Self> {
        let cfg = AppConfig::new().await?; 
        let jobs_file_path = "jobs.json";
        let nodes_file_path = "nodes.json";
        println!("[INFO] Attempting to load jobs from: {}", jobs_file_path);
        let jobs_content = fs::read_to_string(jobs_file_path)?;
        let loaded_jobs: Vec<Job> = serde_json::from_str(&jobs_content)?;
        println!("[INFO] Successfully loaded {} initial jobs.", loaded_jobs.len());

        println!("[INFO] Attempting to load nodes from: {}", nodes_file_path);
        let nodes_content = fs::read_to_string(nodes_file_path)?;
        let loaded_nodes: Vec<Node> = serde_json::from_str(&nodes_content)?;
        println!("[INFO] Successfully loaded {} initial nodes.", loaded_nodes.len());

        Ok(Self {
            jobs: Arc::new(Mutex::new(loaded_jobs)),
            nodes: Arc::new(Mutex::new(loaded_nodes)),
            cfg: Arc::new(cfg),
        })
    }
}
