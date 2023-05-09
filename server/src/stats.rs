use redis::{Cmd, ConnectionLike};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::collections::HashMap;

use crate::{
    errors::{Error, ErrorTy, Result, SrcRedis},
    redis2::SetType,
    AppState,
};

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
/// This may be used in areas where other i64s will be used
pub struct JobStatus(pub i64);

#[derive(Debug, Serialize, Deserialize)]
pub struct Stats {
    // a hashmap of job statuses and the amount of jobs under each (i.e. ["successful", 34])
    pub job_stats: HashMap<i64, i64>,
    // (<name>,<id>)
    pub connected_runners: Vec<(String, i64)>,
}

impl Stats {
    pub async fn get(app: &AppState) -> Result<Self> {
        let mut redis_client = app.cache.redis.clone().0.get_connection()?;
        match redis_client.req_command(&Cmd::get(&"ChannelCi-Stats".to_string())) {
            Ok(val) => match val {
                redis::Value::Nil => {
                    let jobs = sqlx::query_as::<_, JobStatus>(r#"SELECT (status) FROM job"#)
                        .fetch_all(&app.database.0)
                        .await?;

                    let mut job_stats: HashMap<i64, i64> = HashMap::new();
                    let connected_runners = Vec::new();
                    for job_status in jobs {
                        job_stats.insert(
                            job_status.0,
                            match job_stats.get(&job_status.0) {
                                Some(s) => s + 1,
                                None => 1,
                            },
                        );
                    }

                    let stats = Stats {
                        job_stats,
                        connected_runners,
                    };
                    app.cache
                        .redis
                        .set("ChannelCi-Stats", &stats, SetType::WithTTL(3600))?;
                    Result::Ok(stats)
                }
                redis::Value::Data(data) => Result::Ok(serde_json::from_slice::<Stats>(&data)?),
                _ => Result::Err(Error {
                    source: &SrcRedis,
                    ty: ErrorTy::Unkown,
                    msg: Some(String::from("Failed to get data from redis")),
                }),
            },
            Err(_err) => Result::Err(Error {
                source: &SrcRedis,
                ty: ErrorTy::Unkown,
                msg: Some(String::from("Error getting stats from Redis")),
            }),
        }
    }
}
