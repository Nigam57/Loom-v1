use axum::{
    routing::{get, post},
    extract::{Path, State},
    response::sse::{Event, Sse},
    Router, Json,
};
use std::sync::{Arc, Mutex};
use std::convert::Infallible;
use tokio::sync::broadcast;
use serde::Deserialize;
use crate::pty::manager::PtyManager;
use crate::pty::commands::PtyEvent;
use std::collections::HashMap;
use futures::Stream;

#[derive(Clone)]
pub struct ServerState {
    pub pty_manager: Arc<Mutex<PtyManager>>,
    pub broadcaster: Arc<Mutex<HashMap<u32, broadcast::Sender<PtyEvent>>>>,
}

#[derive(Deserialize)]
pub struct WritePayload {
    pub data: String,
}

pub async fn start_server(port: u16, state: ServerState) {
    let addr = format!("127.0.0.1:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    start_server_with_listener(listener, state).await;
}

pub async fn start_server_with_listener(listener: tokio::net::TcpListener, state: ServerState) {
    let app = Router::new()
        .route("/pty/:pid/write", post(write_pty))
        .route("/pty/:pid/events", get(pty_events))
        .with_state(state);

    axum::serve(listener, app).await.unwrap();
}

async fn write_pty(
    Path(pid): Path<u32>,
    State(state): State<ServerState>,
    Json(payload): Json<WritePayload>,
) -> Result<&'static str, (axum::http::StatusCode, String)> {
    let manager = state.pty_manager.lock().unwrap();
    manager.write_pty(pid, &payload.data)
        .map(|_| "OK")
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e))
}

async fn pty_events(
    Path(pid): Path<u32>,
    State(state): State<ServerState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = {
        let mut map = state.broadcaster.lock().unwrap();
        if let Some(tx) = map.get(&pid) {
            tx.subscribe()
        } else {
            let (tx, rx) = broadcast::channel(100);
            map.insert(pid, tx);
            rx
        }
    };

    let stream = futures::stream::unfold(rx, |mut rx| async move {
        loop {
            match rx.recv().await {
                Ok(event) => {
                    let json = serde_json::to_string(&event).unwrap_or_default();
                    return Some((Ok(Event::default().data(json)), rx));
                }
                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                Err(tokio::sync::broadcast::error::RecvError::Closed) => return None,
            }
        }
    });

    Sse::new(stream).keep_alive(axum::response::sse::KeepAlive::default())
}
