use anyhow::Result;
use async_trait::async_trait;

use crate::AppState;

#[async_trait]
pub trait CacheAble: Sized {
    type GetReturn = Self;
    async fn get_with_i64(_: &AppState, _: i64) -> Result<Self::GetReturn> {
        unimplemented!()
    }
    async fn get_using_name_and_owner(_: &AppState, _: &str, _: &str) -> Result<Self::GetReturn> {
        unimplemented!()
    }
    async fn insert_self(&self, _: &AppState) -> Result<()> {
        unimplemented!()
    }
}
