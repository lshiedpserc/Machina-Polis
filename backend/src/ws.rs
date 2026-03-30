use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, Path, State},
    response::IntoResponse,
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::sync::Arc;
use tokio::sync::mpsc;

use crate::state::{AppState, SessionId};
use crate::models::{HumanMessageRequest, Message as ChatMessage, SessionStatus};

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Path(session_id): Path<SessionId>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, session_id, state))
}

async fn handle_socket(socket: WebSocket, session_id: SessionId, state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();

    // Create an unbounded channel to forward messages to this specific client
    let (tx, mut rx) = mpsc::unbounded_channel();

    // Register the client sender with the application state
    state.add_client(session_id.clone(), tx).await;

    // Send the full history upon connection
    if let Some(session_arc) = state.get_session(&session_id).await {
        let session = session_arc.read().await;
        for message in &session.messages {
            if let Ok(json) = serde_json::to_string(message) {
                let _ = sender.send(Message::Text(json.into())).await;
            }
        }
    } else {
        let _ = sender.send(Message::Text("Session not found".into())).await;
        return;
    }

    // Spawn a task to send messages from the state to the websocket
    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    // Handle incoming messages from the client (e.g. human turn input)
    let state_clone = state.clone();
    let session_id_clone = session_id.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            // Very simple message handling logic
            if let Ok(req) = serde_json::from_str::<HumanMessageRequest>(&text) {
                if let Some(session_arc) = state_clone.get_session(&session_id_clone).await {
                    let mut session = session_arc.write().await;
                    if session.status == SessionStatus::Active {
                        let human_msg = ChatMessage::new_human(req.content);
                        session.add_message(human_msg.clone());

                        // Broadcast to all clients
                        if let Ok(json) = serde_json::to_string(&human_msg) {
                            state_clone.broadcast(&session_id_clone, Message::Text(json.into())).await;
                        }
                    }
                }
            }
        }
    });

    // If any one of the tasks run to completion, we abort the other.
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}