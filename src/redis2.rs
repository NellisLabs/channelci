use crate::{
    config::CONFIG,
    errors::{Error, ErrorTy, Result, SrcRedis},
};
//use anyhow::{bail, Result};
use common::database::Database;
use futures::Future;
use redis::{Client as OriginalClient, Cmd, Connection, ConnectionLike};
use serde::{Deserialize, Serialize};
use std::{ops::Fn, sync::Arc};

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
    pub async fn get<K: Into<String>, R: for<'a> Deserialize<'a> + Serialize, F, Fut>(
        &self,
        key: K,
        db: Option<&Database>,
        fn_if_not_found: Option<F>,
        set_type: Option<SetType>,
    ) -> Result<R>
    where
        F: Fn(Database) -> Fut,
        Fut: Future<Output = R>,
    {
        let mut redis = self.get_conn()?;
        let key = key.into();
        match redis.req_command(&Cmd::get(&key)) {
            Ok(val) => match val {
                redis::Value::Nil => match (db, fn_if_not_found) {
                    (Some(db), Some(fn_if_not_found)) => {
                        let r = fn_if_not_found(db.clone()).await;

                        self.set(&key, &r, set_type.unwrap_or_default())?;
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
