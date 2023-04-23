use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRunnerData {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    pub name: String,
    pub owner: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ManualJobTrigger {
    pub repo: Repository,
    // People can choose what runner a job runs on if they trigger it manually.
    // more about this in the next episode.
    pub runner: Option<String>,
}
