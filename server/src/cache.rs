use crate::{errors::Result, redis2::Client};
use common::database::Database;

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
}
