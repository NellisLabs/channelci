use crate::errors::{Error, ErrorTy, SrcRedis, SrcUnkown};
use crate::AppState;
use crate::{cacheable::CacheAble, errors::Result, stats::JobStatus};
use channel_common::models::Runners;
use rand::seq::SliceRandom;
use rand::Rng;
use redis::{Cmd, ConnectionLike};

pub fn random_from_vec<T: ToOwned<Owned = T>>(vec: Vec<T>) -> Result<T> {
    if let Some(random) = vec.choose(&mut rand::thread_rng()) {
        return Result::Ok(random.to_owned());
    }
    Result::Err(Error {
        source: &SrcUnkown,
        ty: ErrorTy::Unkown,
        msg: Some(String::from(
            "Failed to select a random element from the provided vector.",
        )),
    })
}

pub async fn select_random_runner(app: &AppState) -> Result<Runners> {
    let mut redis = app.redis.get_conn()?;
    let Ok(redis::Value::Int(taken_runners_length)) = redis.req_command(&Cmd::llen("ChannelCi-Taken-Runners")) else {
        return Result::Err(Error {
            source: &SrcRedis,
            ty: ErrorTy::Unkown,
            msg: Some(String::from("Failed to get length of ChannelCi-Taken-Runners key out of redis.")),
        });
    };
    let taken_runners_length: u64 = taken_runners_length.try_into()?;
    match taken_runners_length {
        // There are no taken runners and thus no jobs being ran. We can select any at random.
        0 => {
            let all_runners = sqlx::query_as::<_, JobStatus>("SELECT id FROM runners")
                .fetch_all(&app.database.0)
                .await?;

            println!("{all_runners:?}");
            let mut runners = Vec::new();
            for runner in &all_runners {
                let runner = Runners::get_with_i64(app, runner.0).await?;
                println!("Possible Job: {runner:?}");
                runners.push(runner)
            }
            println!("Runners Lenth: {runners:?}");
            if runners.len() == 1 {
                return Result::Ok(runners.get(0).unwrap().to_owned());
            }
            random_from_vec(runners)
        }
        trl @ 1.. => {
            let random_runner: isize = rand::thread_rng().gen_range(0..trl).try_into()?;
            let Ok(redis::Value::Data(taken_runners_length)) = redis.req_command(&Cmd::lrange("ChannelCi-Taken-Runners", random_runner, random_runner)) else {
                return Result::Err(Error {
                    source: &SrcRedis,
                    ty: ErrorTy::Unkown,
                    msg: Some(String::from("Failed to get selected runner from ChannelCi-Taken-Runners in Redis.")),
                });
            };
            Result::Ok(serde_json::from_slice::<Runners>(&taken_runners_length)?)
        }
    }
}
