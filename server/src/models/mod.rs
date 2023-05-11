use crate::{
    db::{DatabaseImpl, DbQuery},
    errors::Result,
};
use async_trait::async_trait;
use common::{database::Database, objects::Projects};
use sqlx::{Encode, Postgres, Type};

#[async_trait]
pub trait DatabaseObjectGetterAndSetters {
    async fn create_project<'a, B: for<'b> Encode<'b, Postgres> + Send + Type<Postgres> + 'a>(
        &self,
        bindings: Vec<B>,
    ) -> Result<Projects>;
}

#[async_trait]
impl DatabaseObjectGetterAndSetters for Database {
    async fn create_project<'a, B: for<'b> Encode<'b, Postgres> + Send + Type<Postgres> + 'a>(
        &self,
        bindings: Vec<B>,
    ) -> Result<Projects> {
        Result::Ok(
            self.insert_return_one::<Projects, B>(
                "INSERT INTO project(name,git_url) VALUES($1,$2) RETURNING *",
                bindings,
            )
            .await?,
        )
    }
}
