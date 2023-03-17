use std::{
    net::{Ipv4Addr, SocketAddr},
    path::PathBuf,
};

use anyhow::Context;
use axum::{
    extract::{ws::WebSocket, State, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use state::AppState;
use tokio::net::UdpSocket;
use tower_http::services::ServeDir;

use crate::{message::Message, state::AppStateExt};

pub(crate) mod database;
pub(crate) mod message;
pub(crate) mod state;

async fn listen_temp(state: AppState) -> anyhow::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:55555").await?;

    socket.join_multicast_v4(Ipv4Addr::new(224, 10, 12, 58), Ipv4Addr::UNSPECIFIED)?;

    tracing::debug!("Listening ...");

    loop {
        let mut buf = [0u8; 4096];
        let (size, addr) = socket.recv_from(&mut buf).await?;

        tracing::debug!("Got {} from {}", size, addr);

        match Message::try_from(&buf[0..size]) {
            Ok(msg) => {
                database::insert_record(state.pool(), &msg).await?;
                if let Err(e) = state.tx().send(msg) {
                    tracing::warn!("Failed to send to websocket : {}", e);
                }
            }
            Err(e) => {
                tracing::error!("Failed to parse temperature message : {}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let db_pool = database::open_connection(
        &std::env::var("DATABASE_URL").context("Failed to get DATABASE_URL")?,
    )
    .await?;

    let state = AppState::new_state(db_pool);

    let s = state.clone();
    tokio::spawn(async {
        listen_temp(s).await.unwrap();
    });

    let assets_dir = PathBuf::from("assets");

    let app = Router::new()
        .fallback_service(ServeDir::new(assets_dir).append_index_html_on_directories(true))
        .route("/ws", get(ws_handler))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn ws_handler(State(state): State<AppState>, ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {
    let mut rx = state.tx().subscribe();

    loop {
        let msg = rx.recv().await.unwrap();
        socket
            .send(serde_json::to_string(&msg).unwrap().into())
            .await
            .unwrap();
    }
}
