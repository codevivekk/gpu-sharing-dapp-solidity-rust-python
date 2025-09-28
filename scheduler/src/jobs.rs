use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use crate::{models::Job, state::AppState};
use actix_web::{web, HttpResponse, Responder};

async fn get_jobs(data: web::Data<AppState>) -> impl Responder {
    let jobs = data.jobs.lock().unwrap();
    HttpResponse::Ok().json(&*jobs)
}

async fn add_job(job: web::Json<Job>, data: web::Data<AppState>) -> impl Responder {
    let mut jobs = data.jobs.lock().unwrap();
    let new_job = job.into_inner();
    jobs.push(new_job.clone());

    let file_path = "jobs.json";

    let mut all_jobs: Vec<Job> = if let Ok(mut file) = File::open(file_path) {
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        if content.is_empty() {
            Vec::new()
        } else {
            serde_json::from_str(&content).unwrap_or_else(|_| Vec::new())
        }
    } else {
        Vec::new()
    };
    all_jobs.push(new_job);
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)
        .unwrap();

    let json = serde_json::to_string_pretty(&all_jobs).unwrap();
    file.write_all(json.as_bytes()).unwrap();

    HttpResponse::Ok().json(&*jobs)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/jobs")
            .route("", web::get().to(get_jobs))
            .route("", web::post().to(add_job)),
    );
}
