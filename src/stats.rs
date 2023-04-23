use anyhow::{bail, Result};
use redis::{Cmd, ConnectionLike};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::collections::HashMap;

use crate::{redis2::SetType, AppState};

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
        let mut redis_client = app.redis.clone().0.get_connection()?;
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
                    app.redis
                        .set("ChannelCi-Stats", &stats, SetType::WithTTL(3600))?;
                    Ok(stats)
                }
                redis::Value::Data(data) => Ok(serde_json::from_slice::<Stats>(&data)?),
                _ => bail!("Invalid response from Redis"),
            },
            Err(_err) => {
                bail!("Error getting stats from Redis")
            }
        }
    }
}
