#![feature(associated_type_defaults)]
#![feature(try_trait_v2)]

pub mod cache;
pub mod config;
pub mod db;
pub mod errors;
pub mod impls;
pub mod ingest;
pub mod objects;
pub mod redis2;
pub mod routes;
pub mod stats;
use std::net::SocketAddr;

use crate::{
    cache::Cache,
    errors::Result,
    routes::{
        get_stats,
        jobs::{get_job, github_webhook},
        objects::create_project,
        runners::{get_runner_current_job, ws_handler},
    },
    stats::Stats,
};
use anyhow::{bail, Result as AnyhowResult};
use axum::{
    routing::{get, post},
    Router,
};
use channel_common::websocket::WebsocketMessage;
use common::database::{self, Database};
use config::CONFIG;
use parking_lot::RwLock;
use redis2::Client;

use std::sync::Arc;
use std::{collections::HashMap, time::Instant};
use tokio::sync::mpsc::UnboundedSender;

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
    pub cache: Cache,
    pub connected_runners: Arc<RwLock<HashMap<String, ConnectedRunner>>>,
}

impl AppState {
    pub async fn init() -> errors::Result<AppState> {
        // TODO: The unwraps here deal with the fact the error trait is defined in this crate and not in the
        // Common crate so the Database struct doesn't implement it and instead implements Anyhow.
        let database = Database::new(CONFIG.database.to_string()).await.unwrap();
        database.migrate().await.unwrap();

        let connected_runners: HashMap<String, ConnectedRunner> = HashMap::new();
        let connected_runners: Arc<RwLock<HashMap<String, ConnectedRunner>>> =
            Arc::new(RwLock::new(connected_runners));

        let cache = Cache::init(database.clone())?;
        errors::Result::Ok(Self {
            cache,
            database,
            connected_runners,
        })
    }
}

#[derive(Debug)]
pub struct ConnectedRunner {
    pub name: String,
    pub sender: UnboundedSender<WebsocketMessage>,
    pub identified: bool,
    pub last_hb: Instant,
}

#[tokio::main]
async fn main() -> AnyhowResult<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .pretty()
        .init();

    let Result::Ok(app_state) = AppState::init().await else {
        bail!("Failed to create AppState");
    };

    // Load some basic statistics for the initial page load, when it happens. This will store for 1 hour then expire
    // until the next request that requires it. As the service is just starting, the connected_runners field will always
    // be empty and won't be updated till runners connect and this expires and gets reset.
    if let Result::Err(_) = Stats::get(&app_state).await {
        bail!("Failed to load stats into redis.");
    }

    let runner_routes = Router::new()
        .route("/:runner/current", get(get_runner_current_job))
        .route("/:runner/connect", get(ws_handler));

    let job_routes = Router::new()
        //.route("/", get(get_jobs))
        .route("/:id", get(get_job));

    let object_routes = Router::new().route("/project", post(create_project));

    let app = Router::new()
        .nest("/runners", runner_routes)
        .nest("/jobs", job_routes)
        .nest("/objects", object_routes)
        .route("/stats", get(get_stats))
        .route("/github", post(github_webhook))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Starting on: {addr:?}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await?;
    println!("Started");
    Ok(())
}
