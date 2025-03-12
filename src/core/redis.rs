use redis::{AsyncCommands, Client};
use anyhow::Result;
use serde_json::Value;

pub struct RedisClient {
    client: Client,
}

impl RedisClient {
    pub async fn new(redis_url: &str) -> Result<Self> {
        let client = Client::open(redis_url)?;
        Ok(Self { client })
    }

    pub async fn get_json<T: serde::de::DeserializeOwned>(&self, key: &str) -> Result<T> {
        let mut con = self.client.get_async_connection().await?;
        let json_str: String = con.get(key).await?;
        let value: T = serde_json::from_str(&json_str)?;
        Ok(value)
    }

    pub async fn set_json<T: serde::Serialize>(&self, key: &str, value: &T) -> Result<()> {
        let mut con = self.client.get_async_connection().await?;
        let json_str = serde_json::to_string(value)?;
        con.set(key, json_str).await?;
        Ok(())
    }

    pub async fn delete(&self, key: &str) -> Result<()> {
        let mut con = self.client.get_async_connection().await?;
        con.del(key).await?;
        Ok(())
    }
}
