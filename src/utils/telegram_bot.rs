use reqwest;
use std::error::Error;

pub struct TelegramBot {
    url: String,
    chat: String,
}

impl TelegramBot {
    pub fn new(token: &str, chat: &str) -> Self {
        let url = format!("https://api.telegram.org/bot{}", token);
        Self {
            url,
            chat: chat.to_string(),
        }
    }

    pub async fn send_message(&self, message: &str) -> Result<(), Box<dyn Error>> {
        let message  = message.to_string();
        let params = [("chat_id", &self.chat), ("text", &message)];
        reqwest::Client::new()
            .post(format!("{}/sendMessage", self.url))
            .form(&params)
            .send()
            .await?;
        Ok(())
    }
}
