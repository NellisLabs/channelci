use crate::{errors::Result, AppState};
use channel_common::models::Repos;
use serde::{Deserialize, Serialize};

pub mod github {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Serialize, Deserialize)]
    pub struct User {
        pub id: i64,
        pub name: Option<String>,
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
