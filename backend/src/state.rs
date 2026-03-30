use axum::extract::ws::{Message as WsMessage, WebSocket};
use futures::stream::SplitSink;
use futures::SinkExt;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};

use crate::models::{Agent, Mode, Session};
use crate::engine::SessionEngine;

pub type SessionId = String;

pub struct AppState {
    pub sessions: RwLock<HashMap<SessionId, Arc<RwLock<Session>>>>,
    pub active_engines: RwLock<HashMap<SessionId, Arc<SessionEngine>>>,
    pub clients: RwLock<HashMap<SessionId, Vec<mpsc::UnboundedSender<WsMessage>>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
            active_engines: RwLock::new(HashMap::new()),
            clients: RwLock::new(HashMap::new()),
        }
    }

    pub async fn add_session(&self, id: SessionId, session: Session) -> Arc<RwLock<Session>> {
        let session = Arc::new(RwLock::new(session));
        self.sessions.write().await.insert(id, session.clone());
        session
    }

    pub async fn get_session(&self, id: &SessionId) -> Option<Arc<RwLock<Session>>> {
        self.sessions.read().await.get(id).cloned()
    }

    pub async fn add_client(&self, session_id: SessionId, sender: mpsc::UnboundedSender<WsMessage>) {
        let mut clients = self.clients.write().await;
        clients
            .entry(session_id)
            .or_insert_with(Vec::new)
            .push(sender);
    }

    pub async fn broadcast(&self, session_id: &SessionId, message: WsMessage) {
        if let Some(senders) = self.clients.read().await.get(session_id) {
            for sender in senders {
                let _ = sender.send(message.clone());
            }
        }
    }
}
