use crate::{cacheable::CacheAble, redis2::SetType, AppState};
use anyhow::{bail, Result};
use async_trait::async_trait;
use channel_common::{database::Database, models::Repos};
use redis::{Cmd, ConnectionLike};

#[async_trait]
impl CacheAble for Repos {
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
