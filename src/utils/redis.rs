use redis::AsyncCommands;
use redis::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
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
    pub async fn new_redis_chat(session_id: String, redis_url: &str) -> Result<Self, Box<dyn Error>> {
        let client = Client::open(redis_url)?;
        Ok(Self { client, session_id })
    }

    pub async fn base_messages(&self) -> Result<Vec<RedisChatMessage>, Box<dyn Error>> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let messages: Option<String> = conn.get(&self.session_id).await?;

        if let Some(data) = messages {
            let decoded_messages: Vec<RedisChatMessage> = serde_json::from_str(&data)?;
            Ok(decoded_messages)
        } else {
            Ok(vec![])
        }
    }

    pub async fn add_message(&self, role: &str, content: &str) -> Result<(), Box<dyn Error>> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let mut messages = self.base_messages().await?;
        messages.push(RedisChatMessage {
            role: role.to_string(),
            content: content.to_string(),
        });
        let serialized = serde_json::to_string(&messages)?;
        conn.set::<_, _, ()>(&self.session_id, serialized).await?;
        Ok(())
    }
}

pub struct RedisClient {
    client: Arc<Mutex<Client>>,
}

impl RedisClient {
    pub async fn new_redis_client(redis_url: &str) -> Result<Self, Box<dyn Error>> {
        let client = Client::open(redis_url)?;
        Ok(Self {
            client: Arc::new(Mutex::new(client)),
        })
    }

    pub async fn get_connection(&self) -> Result<redis::aio::MultiplexedConnection, Box<dyn Error>> {
        let client = self.client.lock().await;
        let conn = client.get_multiplexed_async_connection().await?;
        Ok(conn)
    }
}