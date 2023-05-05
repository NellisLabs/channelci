use crate::{
    cacheable::CacheAble, errors::Result, stats::JobStatus, AppState,
};

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use channel_common::models::Job;

use serde_json::json;

pub async fn get_job(
    State(app): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse> {
    let job = Job::get_with_i64(&app, id).await?;
    Result::Ok((StatusCode::OK, json!({ "job": job }).to_string()))
}

pub async fn get_jobs(State(app): State<AppState>) -> Result<impl IntoResponse> {
    let all_jobs = sqlx::query_as::<_, JobStatus>("SELECT id FROM job ORDER BY start ASC")
        .fetch_all(&app.database.0)
        .await?;

    let mut jobs = Vec::new();
    for job in &all_jobs {
        if let Result::Ok(job) = Job::get_with_i64(&app, job.0).await {
            jobs.push(job)
        }
    }

    Result::Ok((StatusCode::OK, serde_json::to_string(&jobs)?))
}

pub async fn manual_job_trigger(
    State(_app): State<AppState>,
    //Json(_job): Json<ManualJobTrigger>,
) -> impl IntoResponse {
    // let Ok(repo) = Repos::get_using_name_and_owner(&app, &job.repo.name, &job.repo.owner).await else {
    //     return (
    //         StatusCode::NOT_FOUND,
    //         json!({"msg": "Could not find the requested repo."}).to_string(),
    //     )
    // };
    // let Ok(pipeline) = Pipelines::get_with_i64(&app, repo.id).await else {
    //     return (
    //         StatusCode::NOT_FOUND,
    //         json!({"msg": "Could not find the requested pipeline."}).to_string(),
    //     )
    // };
    // let Ok(steps) = sqlx::query_as::<_, PipelineStep>(r#"SELECT * FROM pipeline_step WHERE belongs_to = ($1)"#).bind(pipeline.id).fetch_all(&app.database.0).await else {
    //     return     (
    //         StatusCode::INTERNAL_SERVER_ERROR,
    //         json!({"3":"3"}).to_string(),
    //     );
    // };

    // let Ok(job) = create_job(&app, job.repo, None, "Manual Trigger").await else {
    //     return     (
    //         StatusCode::INTERNAL_SERVER_ERROR,
    //         json!({"msg":"Failed to create job."}).to_string(),
    //     );
    // };
    // app.redis
    //     .set(
    //         format!("ChannelCi-Job:{}", job.id),
    //         &job,
    //         SetType::WithTTL(3600),
    //     )
    //     .unwrap();

    // let runners = app.connected_runners.read();
    // let id = job.id;
    // let connected_runner = runners.get(&runner.0.clone()).unwrap();
    // connected_runner
    //     .sender
    //     .send(channel_common::websocket::WebsocketMessage {
    //         op: channel_common::websocket::OpCodes::EventCreate,
    //         event: Some(Box::new(channel_common::events::CreateJobRun {
    //             job,
    //             pipeline,
    //             steps,
    //         })),
    //     })
    //     .unwrap();
    // drop(runners);
    (StatusCode::OK, json!({}).to_string()) //json!({ "job_id": id }).to_string())
}
