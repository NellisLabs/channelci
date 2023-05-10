use std::{any::Any, process::Output};

use crate::{
    errors::Result,
    redis2::{Client, DummyFuture, SetType},
};
use channel_common::models::{Job, Runners};
use common::{
    database::Database,
    objects::{Objects, Pipelines, Projects},
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow};

#[derive(Clone)]
pub struct Cache {
    pub redis: Client,
    pub database: Database,
}

impl Cache {
    pub fn init(database: Database) -> Result<Self> {
        Result::Ok(Self {
            database,
            redis: Client::new()?,
        })
    }
    pub async fn get_job(&self, id: &str) -> Result<Job> {
        Result::Ok(self.get("job", id, Some(SetType::WithTTL(3600))).await?)
    }
    pub async fn get_runner(&self, id: &str) -> Result<Runners> {
        Result::Ok(self.get("runner", id, Some(SetType::WithTTL(3600))).await?)
    }
    //get("pipelines", "id", 1, Some(&database), None, Some(SetType::))
    pub async fn get<R, K: Into<String>>(
        &self,
        key: K,
        key_val: K,
        set_type: Option<SetType>,
    ) -> Result<R>
    where
        R: for<'a> Deserialize<'a> + Serialize + Unpin + Send + Sized + for<'r> FromRow<'r, PgRow>,
    {
        Result::Ok(
            self.redis
                .get::<R, String, fn(Database) -> DummyFuture<R>, DummyFuture<R>>(
                    key.into(),
                    "id".into(),
                    key_val.into(),
                    Some(&self.database),
                    None,
                    set_type,
                )
                .await?,
        )
    }
    //pub async fn get<K: Into<String>>(&self, key: K)
    // pub async fn get_runner(&self, id: i64) -> Result<Runners> {
    //     Result::Ok(
    //         self.redis
    //             .get(
    //                 format!("ChannelCi-Runner:{}", id),
    //                 Some(&self.database),
    //                 Some(|db: Database| async move {
    //                     match sqlx::query_as::<_, Runners>(
    //                         r#"SELECT * FROM runners WHERE id = ($1)"#,
    //                     )
    //                     .bind(id)
    //                     .fetch_one(&db.0)
    //                     .await
    //                     {
    //                         Ok(runner) => runner,
    //                         Err(_) => panic!("Failed to get runner from database. #{id}"),
    //                     }
    //                 }),
    //                 Some(SetType::WithTTL(3600)),
    //             )
    //             .await?,
    //     )
    // }
    // pub async fn get_object(&self, id: i64) -> Result<Objects> {
    //     Result::Ok(
    //         self.redis
    //             .get(
    //                 format!("ChannelCi-Objects:{}", id),
    //                 Some(&self.database),
    //                 Some(|db: Database| async move {
    //                     match sqlx::query_as::<_, Objects>(
    //                         r#"SELECT * FROM objects WHERE id = ($1)"#,
    //                     )
    //                     .bind(id)
    //                     .fetch_one(&db.0)
    //                     .await
    //                     {
    //                         Ok(object) => object,
    //                         Err(_) => panic!("Failed to get object from database."),
    //                     }
    //                 }),
    //                 Some(SetType::WithTTL(3600)),
    //             )
    //             .await?,
    //     )
    // }
    // pub async fn get_job(&self, id: i64) -> Result<Job> {
    //     Result::Ok(
    //         self.redis
    //             .get(
    //                 format!("ChannelCi-Job:{}", id),
    //                 Some(&self.database),
    //                 Some(|db: Database| async move {
    //                     match sqlx::query_as::<_, Job>(r#"SELECT * FROM job WHERE id = ($1)"#)
    //                         .bind(id)
    //                         .fetch_one(&db.0)
    //                         .await
    //                     {
    //                         Ok(job) => job,
    //                         Err(_) => panic!("Failed to get job from database."),
    //                     }
    //                 }),
    //                 Some(SetType::WithTTL(3600)),
    //             )
    //             .await?,
    //     )
    // }

    // pub async fn get_pipeline(&self, id: i64) -> Result<Pipelines> {
    //     Result::Ok(
    //         self.redis
    //             .get(
    //                 format!("ChannelCi-Pipelines:{}", id),
    //                 Some(&self.database),
    //                 Some(|db: Database| async move {
    //                     match sqlx::query_as::<_, Pipelines>(
    //                         r#"SELECT * FROM pipelines WHERE id = ($1)"#,
    //                     )
    //                     .bind(id)
    //                     .fetch_one(&db.0)
    //                     .await
    //                     {
    //                         Ok(object) => object,
    //                         Err(_) => panic!("Failed to get object from database."),
    //                     }
    //                 }),
    //                 Some(SetType::WithTTL(3600)),
    //             )
    //             .await?,
    //     )
    // }
}
