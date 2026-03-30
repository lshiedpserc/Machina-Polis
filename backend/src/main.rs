pub mod engine;
pub mod models;
pub mod state;
pub mod ws;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use uuid::Uuid;

use models::{Agent, CreateSessionRequest, CreateSessionResponse, Mode, Session};
use state::AppState;
use engine::SessionEngine;

#[tokio::main]
async fn main() {
    println!("Machina Polis Backend Starting...");

    let state = Arc::new(AppState::new());

    let app = Router::new()
        .route("/api/sessions", post(create_session))
        .route("/api/sessions/{id}", get(get_session))
        .route("/api/sessions/{id}/start", post(start_session))
        .route("/ws/{id}", get(ws::ws_handler))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

async fn create_session(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateSessionRequest>,
) -> impl IntoResponse {
    let session_id = Uuid::new_v4().to_string();

    // Convert mode string to enum
    let mode = match payload.mode.as_str() {
        "random" => Mode::ObservationRandom,
        "topic" => Mode::ObservationTopic,
        "discussion" => Mode::ObservationDiscussion,
        "auto" => Mode::ProblemSolvingAuto,
        "self" => Mode::ProblemSolvingSelf,
        _ => Mode::ObservationTopic, // fallback
    };

    // Create agents based on payload
    let mut agents = Vec::new();
    for model in payload.models {
        let role = payload.roles.as_ref().and_then(|r| r.get(&model).cloned());
        agents.push(Agent {
            id: Uuid::new_v4().to_string(),
            name: match model.as_str() {
                "gpt-4o" => "GPT-4 Omni".to_string(),
                "claude-3-5" => "Claude 3.5 Sonnet".to_string(),
                "gemini-1-5" => "Gemini 1.5 Pro".to_string(),
                _ => "Unknown AI".to_string(),
            },
            role,
            model,
        });
    }

    let title = payload.topic.chars().take(30).collect::<String>() + "...";

    let session = Session::new(
        session_id.clone(),
        title,
        Some(payload.topic),
        mode,
        agents,
    );

    state.add_session(session_id.clone(), session).await;

    // Create engine but don't start it yet
    let engine = SessionEngine::new(session_id.clone(), state.clone());
    state.active_engines.write().await.insert(session_id.clone(), Arc::new(engine));

    (StatusCode::CREATED, Json(CreateSessionResponse { session_id }))
}

async fn get_session(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    if let Some(session) = state.get_session(&id).await {
        let s = session.read().await;
        (StatusCode::OK, Json(s.clone())).into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

async fn start_session(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    if let Some(session) = state.get_session(&id).await {
        {
            let mut s = session.write().await;
            s.status = models::SessionStatus::Active;

            // Add initial system message if empty
            if s.messages.is_empty() {
                s.add_message(models::Message::new_system("セッションが開始されました。".to_string()));
            }
        }

        if let Some(engine) = state.active_engines.read().await.get(&id) {
            engine.start();
        }

        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}