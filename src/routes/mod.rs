use axum::{extract::State, response::IntoResponse};
use reqwest::StatusCode;
use serde_json::json;

use crate::{stats::Stats, AppState};

pub mod jobs;
pub mod runners;

pub async fn get_stats(State(app): State<AppState>) -> impl IntoResponse {
    let Ok(stats) = Stats::get(&app).await else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"msg":"Failed to load stats."}).to_string(),
        );
    };
    (
        StatusCode::OK,
        match serde_json::to_string(&stats) {
            Ok(stats) => stats,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    json!({"msg":"Failed to convert stats to string."}).to_string(),
                )
            }
        },
    )
}
