use channel_common::models::Job;

use crate::{
    db::DatabaseImpl, errors::Result, ingest::runners::select_random_runner, stats::JobStatus,
    AppState,
};

pub async fn create_job(
    app: &AppState,
    repo: i64, /* user: ...user type !! FOR FUTURE USE */
    specific_runner: Option<String>,
    creator: &str,
) -> Result<Job> {
    let runner = if let Some(specific_runner) = specific_runner {
        // TODO: Maybe store a Runner name -> ID Map somewhere so we can save one less database call.
        let runner_id = app
            .database
            .execute_one(crate::db::gen_query::<JobStatus, _>(
                "SELECT id FROM runners WHERE name = ($1)",
                true,
                specific_runner,
            ))
            .await?;

        app.cache.get_runner(&runner_id.0.to_string()).await?
    } else {
        select_random_runner(app).await?
    };
    // maybe this could be some function call that just checks a repo exists without getting it
    let repo = app.cache.get_job(repo).await?;
    Result::Ok(
        app.database
            .execute_one(crate::db::gen_query_3::<Job, _, _, _>(
                "INSERT INTO job(assigned_runner,repo,triggered_by) VALUES($1,$2,$3) RETURNING *",
                true,
                &runner.name,
                repo.id,
                creator,
            ))
            .await?,
    )
}
