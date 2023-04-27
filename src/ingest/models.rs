pub use crate::requests::ManualJobTrigger;
use crate::{cacheable::CacheAble, AppState};
use anyhow::Result;
use channel_common::models::Repos;
use serde::{Deserialize, Serialize};

pub mod github {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Serialize, Deserialize)]
    pub struct User {
        pub id: i64,
        pub name: String,
        pub login: String,
    }
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Repo {
        pub id: i64,
        pub name: String,
        pub full_name: String,
        pub private: bool,
        pub owner: Option<User>,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GithubWebhook {
    pub repository: github::Repo,
}

impl GithubWebhook {
    pub async fn get_saved_db(&self, app: &AppState) -> Result<Repos> {
        Ok(Repos::get_with_github_id(app, self.repository.id).await?)
    }
}
