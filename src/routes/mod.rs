use axum::{extract::State, response::IntoResponse};
use reqwest::StatusCode;
use serde_json::json;

use crate::{errors::Result, stats::Stats, AppState};

pub mod objects;
pub mod jobs;
pub mod runners;

pub async fn get_stats(State(app): State<AppState>) -> Result<impl IntoResponse> {
    Result::Ok((
        StatusCode::OK,
        serde_json::to_string(&Stats::get(&app).await?)?,
    ))
}
