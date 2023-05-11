use crate::errors::Result;
use async_trait::async_trait;
use common::database::Database;
use sqlx::{
    postgres::{PgArguments, PgRow, Postgres},
    query::{Query, QueryAs},
    Encode, FromRow, Type,
};

// OMG IMPORTANT COMMENT
// this implementation of the database is extremely new to me so it may be worse than i thought

pub enum DbQuery<'a, R> {
    QueryAs(QueryAs<'a, Postgres, R, PgArguments>),
    Query(Query<'a, Postgres, PgArguments>),
}

#[async_trait]
pub trait DatabaseImpl {
    async fn gen_query<'a, R, B: for<'b> Encode<'b, Postgres> + Send + Type<Postgres> + 'a>(
        &self,
        q: &'a str,
        bindings: Vec<B>,
        returning_q: bool,
    ) -> DbQuery<'a, R>
    where
        R: Unpin + Send + Sized + for<'r> FromRow<'r, PgRow>;

    async fn insert_return_all<R, B: for<'a> Encode<'a, Postgres> + Send + Type<Postgres>>(
        &self,
        q: &str,
        bindings: Vec<B>,
    ) -> Result<Vec<R>>
    where
        R: Unpin + Send + Sized + for<'r> FromRow<'r, PgRow>;

    async fn insert_return_one<R, B: for<'a> Encode<'a, Postgres> + Send + Type<Postgres>>(
        &self,
        q: &str,
        bindings: Vec<B>,
    ) -> Result<R>
    where
        R: Unpin + Send + Sized + for<'r> FromRow<'r, PgRow>;

    async fn insert<R, B: for<'a> Encode<'a, Postgres> + Send + Type<Postgres>>(
        &self,
        q: &str,
        bindings: Vec<B>,
    ) -> Result<()>
    where
        R: Unpin + Send + Sized + for<'r> FromRow<'r, PgRow>;
}

#[async_trait]
impl DatabaseImpl for Database {
    async fn gen_query<'a, R, B: for<'b> Encode<'b, Postgres> + Send + Type<Postgres> + 'a>(
        &self,
        q: &'a str,
        bindings: Vec<B>,
        returning_q: bool,
    ) -> DbQuery<'a, R>
    where
        R: Unpin + Send + Sized + for<'r> FromRow<'r, PgRow>,
    {
        let mut query = if returning_q {
            DbQuery::QueryAs(sqlx::query_as::<Postgres, R>(q))
        } else {
            DbQuery::Query(sqlx::query::<Postgres>(q))
        };
        for bind in bindings {
            query = match query {
                DbQuery::QueryAs(q) => DbQuery::QueryAs(q.bind::<B>(bind)),
                DbQuery::Query(q) => DbQuery::Query(q.bind::<B>(bind)),
            };
        }
        query
    }

    async fn insert<R, B: for<'a> Encode<'a, Postgres> + Send + Type<Postgres>>(
        &self,
        q: &str,
        bindings: Vec<B>,
    ) -> Result<()>
    where
        R: Unpin + Send + Sized + for<'r> FromRow<'r, PgRow>,
    {
        let query = self.gen_query::<R, B>(q, bindings, false).await;
        match query {
            DbQuery::Query(q) => q.execute(&self.0).await?,
            _ => unreachable!(),
        };
        Result::Ok(())
    }

    async fn insert_return_one<R, B: for<'a> Encode<'a, Postgres> + Send + Type<Postgres>>(
        &self,
        q: &str,
        bindings: Vec<B>,
    ) -> Result<R>
    where
        R: Unpin + Send + Sized + for<'r> FromRow<'r, PgRow>,
    {
        let query = self.gen_query::<R, B>(q, bindings, true).await;
        match query {
            DbQuery::QueryAs(q) => Result::Ok(q.fetch_one(&self.0).await?),
            _ => unreachable!(),
        }
    }

    async fn insert_return_all<R, B: for<'a> Encode<'a, Postgres> + Send + Type<Postgres>>(
        &self,
        q: &str,
        bindings: Vec<B>,
    ) -> Result<Vec<R>>
    where
        R: Unpin + Send + Sized + for<'r> FromRow<'r, PgRow>,
    {
        let query = self.gen_query::<R, B>(q, bindings, true).await;
        match query {
            DbQuery::QueryAs(q) => Result::Ok(q.fetch_all(&self.0).await?),
            _ => unreachable!(),
        }
    }
}
