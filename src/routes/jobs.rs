use std::sync::Arc;

use crate::{
    cacheable::CacheAble, redis2::SetType, requests::ManualJobTrigger, stats::JobStatus, AppState,
};
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use channel_common::models::{Job, PipelineStep, Pipelines, Repos, Runners};
use futures::{future::join_all, stream};
use futures::{
    sink::SinkExt,
    stream::{Stream, StreamExt},
};
use parking_lot::RwLock;
use rand::seq::SliceRandom;
use redis::{Cmd, ConnectionLike};
use serde_json::json;

pub async fn get_job(State(app): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    let Ok(job) = Job::get_with_i64(&app, id).await else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({}).to_string(),
        );
    };
    (StatusCode::OK, json!({ "job": job }).to_string())
}

pub async fn get_jobs(State(app): State<AppState>) -> impl IntoResponse {
    let Ok(all_jobs) = sqlx::query_as::<_, JobStatus>("SELECT id FROM job ORDER BY start ASC").fetch_all(&app.database.0).await else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({}).to_string(),
        );
    };
    let mut jobs = Vec::new();
    for job in &all_jobs {
        if let Ok(job) = Job::get_with_i64(&app, job.0).await {
            jobs.push(job)
        }
    }
    (
        StatusCode::OK,
        match serde_json::to_string(&jobs) {
            Ok(jobs) => jobs,
            Err(_) => {
                return (
                    StatusCode::OK,
                    json!({"msg": "Failed to serialize jobs."}).to_string(),
                )
            }
        },
    )
}

pub async fn manual_job_trigger(
    State(app): State<AppState>,
    Json(job): Json<ManualJobTrigger>,
) -> impl IntoResponse {
    let Ok(repo) = Repos::get_using_name_and_owner(&app, &job.repo.name, &job.repo.owner).await else {
        return (
            StatusCode::NOT_FOUND,
            json!({"msg": "Could not find the requested repo."}).to_string(),
        )
    };
    let Ok(pipeline) = Pipelines::get_with_i64(&app, repo.id).await else {
        return (
            StatusCode::NOT_FOUND,
            json!({"msg": "Could not find the requested pipeline."}).to_string(),
        )
    };
    let Ok(steps) = sqlx::query_as::<_, PipelineStep>(r#"SELECT * FROM pipeline_step WHERE belongs_to = ($1)"#).bind(pipeline.id).fetch_all(&app.database.0).await else {
        return     (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"3":"3"}).to_string(),
        );
    };

    let runner = match job.runner {
        Some(runner) => {
            let Ok(runner) = sqlx::query_as::<_, Runners>(r#"SELECT * FROM runners WHERE name = ($1)"#).bind(&runner).fetch_one(&app.database.0).await else {
                return     (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    json!({"6":"6"}).to_string(),
                );
            };

            (runner.name, runner.id)
        }
        None => todo!("Create the ability to select random runners. This worked in the last iteration but was very buggy and to be frank, pretty bad.")
    };

    let Ok(job) = sqlx::query_as::<_, Job>(r#"INSERT INTO job(assigned_runner,repo,triggered_by) VALUES($1,$2,$3) RETURNING *"#).bind(&runner.0).bind(repo.id).bind("Manual").fetch_one(&app.database.0).await else {
            return     (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"6":"6"}).to_string(),
            );
        };
    app.redis
        .set(
            format!("ChannelCi-Job:{}", job.id),
            &job,
            SetType::WithTTL(3600),
        )
        .unwrap();

    let runners = app.connected_runners.read();
    let id = job.id;
    let connected_runner = runners.get(&runner.0.clone()).unwrap();
    connected_runner
        .sender
        .send(channel_common::websocket::WebsocketMessage {
            op: channel_common::websocket::OpCodes::EventCreate,
            event: Some(Box::new(channel_common::events::CreateJobRun {
                job,
                pipeline,
                steps,
            })),
        })
        .unwrap();
    drop(runners);
    (StatusCode::OK, json!({ "job_id": id }).to_string())
}
