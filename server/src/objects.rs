use crate::errors::Result;
use common::{database::Database, objects::Objects};

#[repr(i32)]
pub enum ObjectType {
    Project = 0,
    Pipeline = 1,
}

pub async fn create_object(
    db: &Database,
    obj_ty: ObjectType,
    name: Option<String>,
    refers_to: Option<i64>,
) -> Result<Objects> {
    Result::Ok(
        sqlx::query_as::<_, Objects>(
            r#"INSERT INTO objects(name, refers_to, type) VALUES($1,$2,$3) RETURNING *"#,
        )
        .bind(name)
        .bind(refers_to)
        .bind(obj_ty as i32)
        .fetch_one(&db.0)
        .await?,
    )
}
