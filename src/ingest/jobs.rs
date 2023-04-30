use channel_common::models::{Job, Repos, Runners};

use crate::{
    cacheable::CacheAble, ingest::runners::select_random_runner, requests::Repository,
    stats::JobStatus, AppState,
};
use anyhow::Result;

pub async fn create_job(
    app: &AppState,
    repo: Repository, /* user: ...user type !! FOR FUTURE USE */
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
        Runners::get_with_i64(app, runner_id.0).await? as Runners
    } else {
        select_random_runner(app).await?
    };
    // maybe this could be some function call that just checks a repo exists without getting it
    let repo = Repos::get_using_name_and_owner(app, &repo.name, &repo.owner).await? as Repos;
    Ok(sqlx::query_as::<_, Job>(
        r#"INSERT INTO job(assigned_runner,repo,triggered_by) VALUES($1,$2,$3) RETURNING *"#,
    )
    .bind(&runner.name)
    .bind(repo.id)
    .bind(creator)
    .fetch_one(&app.database.0)
    .await?)
}
