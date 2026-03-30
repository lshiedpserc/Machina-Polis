use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};
use crate::models::{Message, Session, SessionStatus, Mode};
use crate::state::{AppState, SessionId};
use axum::extract::ws::Message as WsMessage;
use serde_json;

pub struct SessionEngine {
    session_id: SessionId,
    state: Arc<AppState>,
    cancel_token: tokio::sync::broadcast::Sender<()>,
}

impl SessionEngine {
    pub fn new(session_id: SessionId, state: Arc<AppState>) -> Self {
        let (cancel_token, _) = tokio::sync::broadcast::channel(1);
        Self {
            session_id,
            state,
            cancel_token,
        }
    }

    pub fn start(&self) {
        let session_id = self.session_id.clone();
        let state = self.state.clone();
        let mut rx = self.cancel_token.subscribe();

        tokio::spawn(async move {
            let session_arc = state.get_session(&session_id).await.unwrap();

            loop {
                let status;
                let mode;
                let next_agent_idx;
                let mut current_agents = vec![];

                {
                    let session = session_arc.read().await;
                    status = session.status.clone();
                    mode = session.mode.clone();
                    current_agents = session.agents.clone();
                    // Basic round-robin based on message count
                    next_agent_idx = session.messages.iter().filter(|m| m.agent_id.is_some()).count() % std::cmp::max(1, current_agents.len());
                }

                if status != SessionStatus::Active {
                    // Wait if paused or complete
                    tokio::select! {
                        _ = sleep(Duration::from_millis(500)) => continue,
                        _ = rx.recv() => break,
                    }
                }

                // If Self mode, we might wait for human input, but for now we'll just mock AI responses
                let current_agent = &current_agents[next_agent_idx];

                // Simulate AI processing delay
                tokio::select! {
                    _ = sleep(Duration::from_secs(3)) => {},
                    _ = rx.recv() => break,
                }

                // Double check status hasn't changed during sleep
                let status = session_arc.read().await.status.clone();
                if status != SessionStatus::Active {
                    continue;
                }

                // Generate mock response
                let mock_content = format!("This is a simulated response from {} based on the current context.", current_agent.name);
                let message = Message::new_agent(mock_content, current_agent);

                // Add to session and broadcast
                {
                    let mut session = session_arc.write().await;
                    session.add_message(message.clone());
                }

                if let Ok(json) = serde_json::to_string(&message) {
                    state.broadcast(&session_id, WsMessage::Text(json.into())).await;
                }
            }
        });
    }

    pub fn stop(&self) {
        let _ = self.cancel_token.send(());
    }
}