use std::collections::HashMap;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct Job {
    pub id: i64,
    pub assigned_runner: String,
    pub status: i64,
    pub triggered_by: String,
    pub start: NaiveDateTime,
    pub end: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct Triggers {
    pub id: i64,
    pub trigger_type: i32,
    pub github_repo_id: Option<i64>,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct TriggersUsedBy {
    pub id: i64,
    pub trigger: i64,
    pub owned_by: i64,
}
