use axum::{Router, routing::get, extract::{State, WebSocketUpgrade, ws::WebSocket}, response::IntoResponse};

use crate::state::AppState;

pub(crate) mod model;

mod room;
mod sensor;

pub(crate) fn router() -> Router<AppState> {
    Router::new()
    .route("/ws", get(ws_handler))
        .nest("/sensor", sensor::router())
        .nest("/room", room::router())
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
