use crate::{cache::Cache, errors::Result, redis2::SetType};
use channel_common::models::Job;
use common::database::Database;

impl Cache {
    pub async fn get_job(&self, id: i64) -> Result<Job> {
        Result::Ok(
            self.redis
                .get(
                    format!("ChannelCi-Job:{}", id),
                    Some(&self.database),
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
