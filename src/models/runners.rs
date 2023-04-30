use crate::{cacheable::CacheAble, redis2::SetType, AppState};
use anyhow::Result;
use async_trait::async_trait;
use channel_common::{database::Database, models::Runners};

#[async_trait]
impl CacheAble for Runners {
    async fn get_with_i64(app: &AppState, id: i64) -> Result<Self::GetReturn> {
        Ok(app
            .redis
            .get(
                format!("ChannelCi-Runner:{}", id),
                Some(&app.database),
                Some(|db: Database| async move {
                    match sqlx::query_as::<_, Runners>(r#"SELECT * FROM runners WHERE id = ($1)"#)
                        .bind(id)
                        .fetch_one(&db.0)
                        .await
                    {
                        Ok(runner) => runner,
                        Err(_) => panic!("Failed to get runner from database. #{id}"),
                    }
                }),
                Some(SetType::WithTTL(3600)),
            )
            .await?)
    }
}
