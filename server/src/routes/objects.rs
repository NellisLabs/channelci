use crate::{
    db::DatabaseImpl,
    errors::Result,
    models::DatabaseObjectGetterAndSetters,
    objects::{create_object, ObjectType},
    stats::JobStatus,
    AppState,
};

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use common::objects::{PipelineStep, Pipelines, Projects};

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateProject {
    pub steps: Option<Vec<String>>,
    #[serde(flatten)]
    pub project: Projects,
}

pub async fn create_project(
    State(app): State<AppState>,
    Json(project): Json<CreateProject>,
) -> Result<impl IntoResponse> {
    let project_name = project.project.name.to_owned();
    let new_project = app
        .database
        .execute_one(crate::db::gen_query_2::<Projects, _, _>(
            "INSERT INTO project(name,git_url) VALUES($1,$2) RETURNING *",
            true,
            project_name,
            project.project.git_url,
        ))
        .await?;

    create_object(
        &app.database,
        ObjectType::Project,
        Some(new_project.name.clone()),
        Some(new_project.id),
    )
    .await?;

    // If there are steps then we need to create a new pipeline
    if let Some(steps) = project.steps {
        // TODO: Create pipeline for steps
        let name = format!("{}-steps", project.project.name);
        let mut step_ids = Vec::new();

        // We could easily use a mass insert query if thats faster, idk, do some research
        for step in steps {
            if let std::result::Result::Ok(step) = sqlx::query_as::<_, PipelineStep>(
                r#"INSERT INTO pipeline_step(name,run) VALUES($1,$2) RETURNING *"#,
            )
            .bind("bbs name")
            .bind(&step)
            .fetch_one(&app.database.0)
            .await
            {
                step_ids.push(step.id);
            }
        }

        // TODO: In the future add a flag that denotes this pipeline was created by the system
        let pipeline = app
            .database
            .execute_one(crate::db::gen_query_3::<Pipelines, _, _, _>(
                "INSERT INTO pipelines(name,steps,projects) VALUES($1,$2,$3) RETURNING *",
                true,
                name,
                step_ids,
                vec![new_project.id],
            ))
            .await?;

        create_object(
            &app.database,
            ObjectType::Pipeline,
            Some(pipeline.name),
            Some(pipeline.id),
        )
        .await?;
    }
    Result::Ok((
        StatusCode::OK,
        json!({
            "project": new_project,
        })
        .to_string(),
    ))
}
