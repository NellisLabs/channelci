use crate::errors::Result;
use async_trait::async_trait;
use common::database::Database;
use sqlx::{
    postgres::{PgArguments, PgRow, Postgres},
    query::{Query, QueryAs},
    Encode, FromRow, Type,
};
use std::fmt::Debug;

// OMG IMPORTANT COMMENT
// this implementation of the database is extremely new to me so it may be worse than i thought

macro_rules! impl_tuple_gen_query {
    ($c:ident, $($i:ident -> $t:ident,)*) => {
        pub fn $c<'a, DbQuerySaneTypeIDontKnowWhyINamedItThis, $($t: Encode<'a, Postgres> + Send + Type<Postgres> + 'a + Debug,)+>(
            q: &'a str,
            returning_q: bool,
            $($i: $t),*
        ) -> DbQuery<'a, DbQuerySaneTypeIDontKnowWhyINamedItThis>
        where
        DbQuerySaneTypeIDontKnowWhyINamedItThis: Unpin + Send + Sized + for<'r> FromRow<'r, PgRow>
        {
            let mut query = if returning_q {
                DbQuery::QueryAs(sqlx::query_as::<Postgres, DbQuerySaneTypeIDontKnowWhyINamedItThis>(q))
            } else {
                DbQuery::Query(sqlx::query::<Postgres>(q))
            };
            $(
                query = match query {
                    DbQuery::QueryAs(q) => DbQuery::QueryAs(q.bind::<$t>($i)),
                    DbQuery::Query(q) => DbQuery::Query(q.bind::<$t>($i)),
                };
            )*
            query
        }
    }
}

impl_tuple_gen_query!(
    gen_query,
    arg1 -> A,
);

impl_tuple_gen_query!(
    gen_query_2,
    arg1 -> A,
    arg2 -> B,
);

pub enum DbQuery<'a, R> {
    QueryAs(QueryAs<'a, Postgres, R, PgArguments>),
    Query(Query<'a, Postgres, PgArguments>),
}

#[async_trait]
pub trait DatabaseImpl {
    async fn gen_query<'a, R, B: Encode<'a, Postgres> + Send + Type<Postgres> + 'a>(
        &self,
        q: &'a str,
        bindings: Vec<B>,
        returning_q: bool,
    ) -> DbQuery<'a, R>
    where
        R: Unpin + Send + Sized + for<'r> FromRow<'r, PgRow>;

    async fn execute<'a, R>(&self, q: DbQuery<'a, R>) -> Result<()>
    where
        R: Unpin + Send + Sized + for<'r> FromRow<'r, PgRow>;

    async fn execute_one<'a, R>(&self, q: DbQuery<'a, R>) -> Result<R>
    where
        R: Unpin + Send + Sized + for<'r> FromRow<'r, PgRow>;

    async fn execute_all<'a, R>(&self, q: DbQuery<'a, R>) -> Result<Vec<R>>
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
    async fn execute<'a, R>(&self, q: DbQuery<'a, R>) -> Result<()>
    where
        R: Unpin + Send + Sized + for<'r> FromRow<'r, PgRow>,
    {
        match q {
            DbQuery::Query(q) => {
                q.execute(&self.0).await?;
                Result::Ok(())
            }
            _ => unreachable!(),
        }
    }
    async fn execute_one<'a, R>(&self, q: DbQuery<'a, R>) -> Result<R>
    where
        R: Unpin + Send + Sized + for<'r> FromRow<'r, PgRow>,
    {
        match q {
            DbQuery::QueryAs(q) => Result::Ok(q.fetch_one(&self.0).await?),
            _ => unreachable!(),
        }
    }
    async fn execute_all<'a, R>(&self, q: DbQuery<'a, R>) -> Result<Vec<R>>
    where
        R: Unpin + Send + Sized + for<'r> FromRow<'r, PgRow>,
    {
        match q {
            DbQuery::QueryAs(q) => Result::Ok(q.fetch_all(&self.0).await?),
            _ => unreachable!(),
        }
    }
    async fn gen_query<'a, R, B: Encode<'a, Postgres> + Send + Type<Postgres> + 'a>(
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
