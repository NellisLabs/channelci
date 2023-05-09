use crate::{cache::Cache, errors::Result, redis2::SetType};
use channel_common::models::Runners;
use common::database::Database;

impl Cache {
    pub async fn get_runner(&self, id: i64) -> Result<Runners> {
        Result::Ok(
            self.redis
                .get(
                    format!("ChannelCi-Runner:{}", id),
                    Some(&self.database),
                    Some(|db: Database| async move {
                        match sqlx::query_as::<_, Runners>(
                            r#"SELECT * FROM runners WHERE id = ($1)"#,
                        )
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
                .await?,
        )
    }
}
