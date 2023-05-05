use std::{
    boxed::Box,
    net::SocketAddr,
    time::{Duration, Instant},
};

use crate::{AppState, ConnectedRunner};
use axum::{
    extract::{
        ws::{Message, WebSocket},
        ConnectInfo, Json, Path, State, WebSocketUpgrade,
    },
    http::StatusCode,
    response::IntoResponse,
};
use channel_common::{events::Identify, models::Runners, websocket::WebsocketMessage};
use futures::{sink::SinkExt, stream::StreamExt};
use ring::rand::SecureRandom;
use serde_json::json;
use tokio::sync::mpsc::unbounded_channel;

pub async fn get_runner_current_job(
    State(app): State<AppState>,
    Path(runner): Path<String>,
) -> impl IntoResponse {
    let runners = app.connected_runners;
    let runners = runners.read();

    let _runner = runners.get(&runner).unwrap();
    println!("Sending a fake message to: {:?}", _runner.addr);
    drop(runners);
    (StatusCode::OK, json!({}).to_string())
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(app): State<AppState>,
    Path(runner): Path<String>,
) -> impl IntoResponse {
    println!("`{runner}` at {addr} connected.");
    ws.on_upgrade(move |socket| handle_socket(socket, addr, app, runner))
}

async fn handle_socket(socket: WebSocket, who: SocketAddr, state: AppState, runner: String) {
    let (sender, mut reader) = unbounded_channel::<WebsocketMessage>();
    let sender_1 = sender.clone();

    let runners = state.connected_runners.clone();
    let mut runners = runners.write();

    runners.insert(
        runner.clone(),
        ConnectedRunner {
            addr: who,
            name: runner.clone(),
            sender,
            last_hb: Instant::now(),
            identified: false,
        },
    );
    drop(runners);

    sender_1
        .send(channel_common::websocket::WebsocketMessage {
            op: channel_common::websocket::OpCodes::Hello,
            event: Some(Box::new(channel_common::events::Hello { heartbeat: 10 })),
        })
        .unwrap();

    let (mut sender, mut receiver) = socket.split();

    let _runners = state.connected_runners.clone();
    let runner_name = runner.clone();
    let _heartbeat_task = tokio::spawn(async move {
        let runners = _runners.clone();

        loop {
            let runners = runners.write();
            let runner = runners.get(&runner_name).unwrap();

            if Instant::now().duration_since(runner.last_hb) > Duration::from_secs(10) {
                println!("!! Too late");
            }
            drop(runners);
            std::thread::sleep(Duration::from_secs(10))
        }
    });

    let _send_task = tokio::spawn(async move {
        while let Some(what) = reader.recv().await {
            sender
                .send(Message::Text(serde_json::to_string(&what).unwrap()))
                .await
                .unwrap();
        }
    });
    let runners = state.connected_runners.clone();
    let _recv_task = tokio::spawn(async move {
        let runners = runners.clone();
        while let Some(Ok(what)) = receiver.next().await {
            println!("Message");
            if let Ok(data) = what.into_text() {
                let Ok(message) = serde_json::from_str::<WebsocketMessage>(&data) else {
                    println!("Failed to decode message");
                    continue;
                };
                match message.op {
                    channel_common::websocket::OpCodes::Identify => {
                        println!("Identify event.");
                        let database = state.database.clone();

                        let Some(d) = message.downcast_event::<Identify>() else {
                                println!("Expected some data for identify");
                                return;
                            };

                        println!("{:?}", d);
                        match sqlx::query_as::<_, Runners>(
                            r#"SELECT * FROM runners WHERE token = ($1)"#,
                        )
                        .bind(&d.token)
                        .fetch_one(&database.0)
                        .await
                        {
                            Ok(runner) => {
                                let runners = runners.read();
                                let runner = runners.get(&runner.name).unwrap();
                                if Instant::now().duration_since(runner.last_hb)
                                    > Duration::from_secs(10)
                                {
                                    println!("Too late")
                                }
                                drop(runners);
                            }
                            Err(_) => {}
                        }
                    }
                    channel_common::websocket::OpCodes::HeartBeatAck => {
                        let mut runners = runners.write();
                        let mut runner = runners.get_mut(&runner.clone()).unwrap();
                        runner.last_hb = Instant::now();
                        drop(runners);
                    }
                    _ => {}
                }
            }
        }
    });
    println!("Hi3");
}

// TODO: Bring this back better
// pub async fn create_runner(
//     State(app): State<AppState>,
//     Json(new_runner): Json<CreateRunnerData>,
// ) -> impl IntoResponse {
//     let mut new_runner_token = Vec::new();
//     //let mut new_runner_token: [u8; 32] = [0; 32];
//     let sr = ring::rand::SystemRandom::new();
//     sr.fill(&mut new_runner_token).unwrap();

//     let new_runner_token = hex::encode(new_runner_token);
//     let Ok(query) = sqlx::query_as::<_, Runners>(r#"INSERT INTO runners(name,token) VALUES($1,$2) RETURNING *"#)
//     .bind(&new_runner.name)
//     .bind(&new_runner_token)
//     .fetch_one(&app.database.0)
//     .await else {
//         return     (
//             StatusCode::INTERNAL_SERVER_ERROR,
//             json!({}).to_string(),
//         );
//     };

//     (
//         StatusCode::OK,
//         json!({
//             "id": query.id,
//             "name": query.name
//         })
//         .to_string(),
//     )
// }
