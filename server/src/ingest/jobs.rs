use channel_common::models::Job;

use crate::{errors::Result, ingest::runners::select_random_runner, stats::JobStatus, AppState};

pub async fn create_job(
    app: &AppState,
    repo: i64, /* user: ...user type !! FOR FUTURE USE */
    specific_runner: Option<String>,
    creator: &str,
) -> Result<Job> {
    let runner = if let Some(specific_runner) = specific_runner {
        // TODO: Maybe store a Runner name -> ID Map somewhere so we can save one less database call.
        let runner_id =
            sqlx::query_as::<_, JobStatus>(r#"SELECT id FROM runners WHERE name = ($1)"#)
                .bind(&specific_runner)
                .fetch_one(&app.database.0)
                .await?;
        app.cache.get_runner(runner_id.0).await?
    } else {
        select_random_runner(app).await?
    };
    // maybe this could be some function call that just checks a repo exists without getting it
    let repo = app.cache.get_job(repo).await?;
    Result::Ok(
        sqlx::query_as::<_, Job>(
            r#"INSERT INTO job(assigned_runner,repo,triggered_by) VALUES($1,$2,$3) RETURNING *"#,
        )
        .bind(&runner.name)
        .bind(repo.id)
        .bind(creator)
        .fetch_one(&app.database.0)
        .await?,
    )
}
