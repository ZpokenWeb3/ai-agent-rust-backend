use redis::{AsyncCommands, Client};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct RedisChatMessage {
    role: String,
    content: String,
}

pub struct RedisChatHistory {
    client: Client,
    session_id: String,
}

impl RedisChatHistory {
    pub async fn new(session_id: String, redis_url: &str) -> Result<Self, Box<dyn Error>> {
        let client = Client::open(redis_url)?;
        Ok(Self { client, session_id })
    }

    pub async fn base_messages(&self) -> Result<Vec<RedisChatMessage>, Box<dyn Error>> {
        let mut conn = self.client.get_async_connection().await?;
        let messages: Option<String> = conn.get(&self.session_id).await?;

        if let Some(data) = messages {
            let decoded_messages: Vec<RedisChatMessage> = serde_json::from_str(&data)?;
            Ok(decoded_messages)
        } else {
            Ok(vec![])
        }
    }

    pub async fn add_message(&self, role: &str, content: &str) -> Result<(), Box<dyn Error>> {
        let mut conn = self.client.get_async_connection().await?;
        let mut messages = self.base_messages().await?;
        messages.push(RedisChatMessage {
            role: role.to_string(),
            content: content.to_string(),
        });
        let serialized = serde_json::to_string(&messages)?;
        conn.set(&self.session_id, serialized).await?;
        Ok(())
    }
}
