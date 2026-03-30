use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    ObservationRandom,
    ObservationTopic,
    ObservationDiscussion,
    ProblemSolvingAuto,
    ProblemSolvingSelf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub role: Option<String>,
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MessageRole {
    System,
    Human,
    Agent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub role: MessageRole,
    pub content: String,
    pub agent_id: Option<String>,
    pub agent_name: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl Message {
    pub fn new_system(content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            role: MessageRole::System,
            content,
            agent_id: None,
            agent_name: None,
            timestamp: Utc::now(),
        }
    }

    pub fn new_human(content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            role: MessageRole::Human,
            content,
            agent_id: None,
            agent_name: None,
            timestamp: Utc::now(),
        }
    }

    pub fn new_agent(content: String, agent: &Agent) -> Self {
        Self {
            id: Uuid::new_v4(),
            role: MessageRole::Agent,
            content,
            agent_id: Some(agent.id.clone()),
            agent_name: Some(agent.name.clone()),
            timestamp: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    Initializing,
    Active,
    Paused,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub title: String,
    pub topic: Option<String>,
    pub mode: Mode,
    pub status: SessionStatus,
    pub agents: Vec<Agent>,
    pub messages: Vec<Message>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Session {
    pub fn new(id: String, title: String, topic: Option<String>, mode: Mode, agents: Vec<Agent>) -> Self {
        let now = Utc::now();
        Self {
            id,
            title,
            topic,
            mode,
            status: SessionStatus::Initializing,
            agents,
            messages: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
        self.updated_at = Utc::now();
    }
}

// Request/Response types

#[derive(Debug, Deserialize)]
pub struct CreateSessionRequest {
    pub topic: String,
    pub mode: String,
    pub models: Vec<String>,
    pub roles: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
pub struct CreateSessionResponse {
    pub session_id: String,
}

#[derive(Debug, Deserialize)]
pub struct HumanMessageRequest {
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}
