use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    #[serde(rename = "jobId")]
    pub job_id: String,
    pub owner: String,
    #[serde(rename = "dataset")]
    pub dataset_cid: String,
    #[serde(rename = "containerCID")]
    pub container_cid: String,
    #[serde(deserialize_with = "string_or_float")]
    pub bounty: f64,
    pub deadline: String,
    #[serde(rename = "requiredSpecs")]
    pub required_specs: String,
    #[serde(rename = "minMemory")]
    pub min_memory: u64,
    pub status: String,
    #[serde(default)]
    pub assigned_node: Option<String>,
    #[serde(default)]
    pub provider_address: Option<String>,
    #[serde(default)]
    pub result_hash: Option<String>,
    #[serde(default)]
    pub retries: u8,
    #[serde(default = "default_created_at")]
    pub created_at: String,
    #[serde(default)]
    pub completed: bool,
}


impl Job {
    pub fn new(
        job_id: String,
        owner: String,
        dataset_cid: String,
        container_cid: String,
        bounty: f64,
        deadline: String,
        required_specs: String,
        min_memory: u64,
    ) -> Self {
        Self {
            job_id,
            owner,
            dataset_cid,
            container_cid,
            bounty,
            deadline,
            required_specs,
            min_memory,
            provider_address: None,
            status: "pending".into(),
            assigned_node: None,
            result_hash: None,
            retries: 0,
            created_at: Utc::now().to_rfc3339(),
            completed: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    #[serde(rename = "nodeId")]
    pub node_id: String,
    #[serde(rename = "gpuName")]
    pub gpu_name: Option<String>,
    #[serde(rename = "gpuSpecs")]
    pub gpu_specs: String,
    #[serde(rename = "owner")]
    pub owner: Option<String>,
    #[serde(rename = "memoryAvailable")]
    pub memory: u64,
    #[serde(default)]
    pub status: String, // "idle", "busy"
    #[serde(default)]
    pub active: bool,
}

fn string_or_float<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let v: serde_json::Value = Deserialize::deserialize(deserializer)?;
    match v {
        serde_json::Value::Number(num) => num
            .as_f64()
            .ok_or_else(|| serde::de::Error::custom("invalid number")),
        serde_json::Value::String(s) => s.parse::<f64>().map_err(serde::de::Error::custom),
        _ => Err(serde::de::Error::custom("expected float or string")),
    }
}

fn default_created_at() -> String {
    Utc::now().to_rfc3339()
}
