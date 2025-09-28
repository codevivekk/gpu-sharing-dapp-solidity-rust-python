use std::fs::{File, OpenOptions};
use std::io::Write;
use crate::models::Job;

pub fn save_jobs(jobs: &Vec<Job>) {
    let file_path = "jobs.json";
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)
        .unwrap();
    let json = serde_json::to_string_pretty(jobs).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}