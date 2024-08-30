use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub id: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub actor: Actor,
    pub repo: Repo,
    pub payload: serde_json::Value,
    pub public: bool,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Actor {
    pub id: u64,
    pub login: String,
    pub display_login: String,
    pub gravatar_id: String,
    pub url: String,
    pub avatar_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Repo {
    id: u64,
    pub name: String,
    url: String,
}
