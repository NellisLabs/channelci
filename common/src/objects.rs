use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
/// Represents all possible object types and stores metadata on how to retrieve
/// the respective data
pub struct Objects {
    pub id: i64,
    #[serde(rename = "type")]
    pub obj_type: i32,
    pub name: Option<String>,
    pub refers_to: Option<i64>,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
/// Pipeline object data. This is a bit different from the last
/// iteration of pipelines in that it stories the steps it uses
/// here instead of a constraint from the pipeline_step table.
pub struct Pipelines {
    pub id: i64,
    pub name: String,
    pub steps: Vec<i64>,
    pub flags: i64,
    pub projects: Vec<i64>,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct PipelineStep {
    pub id: i64,
    pub belongs_to: i64,
    pub run: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
/// Reprents a repo object. Bases for most things in Channel
pub struct Repository {
    pub id: i64,
    pub name: String,
    pub steps: Vec<i64>,
    pub git_url: Option<String>,
    pub created_at: NaiveDateTime,
}
