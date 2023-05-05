use crate::{cacheable::CacheAble, errors::Result, redis2::SetType, AppState};
use async_trait::async_trait;
use common::database::Database;
use channel_common::models::Job;

#[async_trait]
impl CacheAble for Job {
    async fn get_with_i64(app: &AppState, id: i64) -> Result<Self::GetReturn> {
        Result::Ok(
            app.redis
                .get(
                    format!("ChannelCi-Job:{}", id),
                    Some(&app.database),
                    Some(|db: Database| async move {
                        match sqlx::query_as::<_, Job>(r#"SELECT * FROM job WHERE id = ($1)"#)
                            .bind(id)
                            .fetch_one(&db.0)
                            .await
                        {
                            Ok(job) => job,
                            Err(_) => panic!("Failed to get job from database."),
                        }
                    }),
                    Some(SetType::WithTTL(3600)),
                )
                .await?,
        )
    }
}
