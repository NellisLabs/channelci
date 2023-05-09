use crate::{cache::Cache, errors::Result, redis2::SetType};
use common::{database::Database, objects::Objects};

impl Cache {
    pub async fn get_object(&self, id: i64) -> Result<Objects> {
        Result::Ok(
            self.redis
                .get(
                    format!("ChannelCi-Objects:{}", id),
                    Some(&self.database),
                    Some(|db: Database| async move {
                        match sqlx::query_as::<_, Objects>(
                            r#"SELECT * FROM objects WHERE id = ($1)"#,
                        )
                        .bind(id)
                        .fetch_one(&db.0)
                        .await
                        {
                            Ok(object) => object,
                            Err(_) => panic!("Failed to get object from database."),
                        }
                    }),
                    Some(SetType::WithTTL(3600)),
                )
                .await?,
        )
    }
}
