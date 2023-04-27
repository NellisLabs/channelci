use crate::{cacheable::CacheAble, redis2::SetType, stats::JobStatus, AppState};
use anyhow::{bail, Result};
use async_trait::async_trait;
use channel_common::{database::Database, models::Repos};
use redis::{Cmd, ConnectionLike};

#[async_trait]
impl CacheAble for Repos {
    async fn get_with_github_id(app: &AppState, id: i64) -> Result<Repos> {
        // The type may be JobStatus but it in fact is just an i64!
        let github_repo_id_to_database_id = app
            .redis
            .get(
                format!("ChannelCi-Repos-GhId-To-DbId:{}", id),
                Some(&app.database),
                Some(|db: Database| async move {
                    match sqlx::query_as::<_, JobStatus>(
                        r#"SELECT id FROM repos WHERE gh_id = ($1)"#,
                    )
                    .bind(&id)
                    .fetch_one(&db.0)
                    .await
                    {
                        Ok(repo) => repo,
                        Err(_) => panic!("Failed to get reposiotyr id from database."),
                    }
                }),
                Some(SetType::WithTTL(1800)),
            )
            .await?;

        Ok(Repos::get_with_i64(app, github_repo_id_to_database_id.0).await?)
    }
    async fn get_using_name_and_owner(app: &AppState, name: &str, owner: &str) -> Result<Repos> {
        Ok(app
            .redis
            .get(
                format!("ChannelCi-Repos:{}-{}", name, owner),
                Some(&app.database),
                Some(|db: Database| async move {
                    match sqlx::query_as::<_, Repos>(
                        r#"SELECT * FROM repos WHERE name = ($1) AND owner = ($2)"#,
                    )
                    .bind(&name)
                    .bind(&owner)
                    .fetch_one(&db.0)
                    .await
                    {
                        Ok(job) => job,
                        Err(_) => panic!("Failed to get repo from database."),
                    }
                }),
                Some(SetType::WithTTL(3600)),
            )
            .await?)
    }

    async fn get_with_i64(app: &AppState, id: i64) -> Result<Repos> {
        Ok(app
            .redis
            .get(
                format!("ChannelCi-Repos:{}", id),
                Some(&app.database),
                Some(|db: Database| async move {
                    match sqlx::query_as::<_, Repos>(r#"SELECT * FROM repos WHERE id = ($1)"#)
                        .bind(&id)
                        .fetch_one(&db.0)
                        .await
                    {
                        Ok(job) => job,
                        Err(_) => panic!("Failed to get repo from database."),
                    }
                }),
                Some(SetType::WithTTL(3600)),
            )
            .await?)
    }
}
