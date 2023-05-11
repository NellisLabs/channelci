use crate::{
    config::CONFIG,
    errors::{Error, ErrorTy, Result, SrcRedis},
};
//use anyhow::{bail, Result};
use common::database::Database;
use futures::{future::Ready, Future};
use redis::{Client as OriginalClient, Cmd, Connection, ConnectionLike};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Encode, FromRow, Postgres, Row, Type};
use std::{ops::Fn, sync::Arc};

pub type DummyFuture<R> = Ready<R>;
pub enum SetType {
    /// field 0 is seconds
    WithTTL(usize),
    Regular,
}

impl Default for SetType {
    fn default() -> Self {
        Self::Regular
    }
}

#[derive(Clone)]
pub struct Client(pub Arc<OriginalClient>);

impl Client {
    pub fn new() -> Result<Self> {
        Result::Ok(Client(Arc::new(OriginalClient::open(
            CONFIG.redis.to_string(),
        )?)))
    }
    pub fn get_conn(&self) -> Result<Connection> {
        Result::Ok(self.0.get_connection()?)
    }
    pub fn set<K: Into<String>, D: Serialize>(
        &self,
        key: K,
        data: D,
        set_type: SetType,
    ) -> Result<()> {
        let mut redis = self.get_conn()?;
        let data = serde_json::to_string(&data)?;
        match set_type {
            SetType::WithTTL(ttl) => _ = redis.req_command(&Cmd::set_ex(key.into(), data, ttl))?,
            SetType::Regular => _ = redis.req_command(&Cmd::set(key.into(), data))?,
        }
        Result::Ok(())
    }
    pub async fn get<'c, R, K: Into<String>, KV, F, Fut>(
        &self,
        key: K,
        key2: K,
        key_value: KV,
        db: Option<&Database>,
        fn_if_not_found: Option<F>,
        set_type: Option<SetType>,
    ) -> Result<R>
    where
        R: for<'a> Deserialize<'a> + Sized + Serialize + Unpin + Send + for<'r> FromRow<'r, PgRow>,
        F: Fn(Database) -> Fut,
        Fut: Future<Output = R>,
        KV: for<'b> Encode<'b, Postgres> + Send + Type<Postgres> + ToString + 'c,
    {
        async fn default_fn_if_not_found<
            'c,
            R: for<'a> Deserialize<'a> + Serialize + Unpin + Send + for<'r> FromRow<'r, PgRow>,
            K: Into<String>,
            KV: for<'b> Encode<'b, Postgres> + Send + Type<Postgres> + 'c,
        >(
            db: &Database,
            key: K,
            key2: K,
            key_value: KV,
        ) -> R {
            match sqlx::query_as::<_, R>(&format!(
                "SELECT * FROM {} WHERE {} = ($1)",
                key.into(),
                key2.into()
            ))
            .bind(key_value)
            .fetch_one(&db.0)
            .await
            {
                Ok(obj) => obj,
                Err(err) => {
                    println!("{err:?}");
                    panic!("OOp")
                }
            }
        }

        let mut redis = self.get_conn()?;
        let key: String = key.into();
        let key_value_str: String = key_value.to_string();
        let formatted_key = format!("ChannelCi-{}:{}", key, key_value_str);
        println!("cannelci --> {formatted_key:?}");
        match redis.req_command(&Cmd::get(&formatted_key)) {
            Ok(val) => match val {
                redis::Value::Nil => match db {
                    Some(db) => {
                        let r = if let Some(fn_if_not_found) = fn_if_not_found {
                            fn_if_not_found(db.clone()).await
                        } else {
                            default_fn_if_not_found(db, key.into(), key2.into(), key_value).await
                        };

                        self.set(&formatted_key, &r, set_type.unwrap_or_default())?;
                        Result::Ok(r)
                    }
                    _ => Result::Err(Error {
                        source: &SrcRedis,
                        ty: ErrorTy::Unkown,
                        msg: Some(String::from("The value returned from Redis was nil.")),
                    }),
                },
                redis::Value::Data(data) => Result::Ok(serde_json::from_slice::<R>(&data)?),
                _ => Result::Err(Error {
                    source: &SrcRedis,
                    ty: ErrorTy::Unkown,
                    msg: Some(String::from("Recieved an invalid response from Redis.")),
                }),
            },
            Err(_err) => Result::Err(Error {
                source: &SrcRedis,
                ty: ErrorTy::Unkown,
                msg: Some(String::from("Could not find the key in Redis.")),
            }),
        }
    }
}
