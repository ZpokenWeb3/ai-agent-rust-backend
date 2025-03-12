use egg_mode::{tweet::DraftTweet, Token};
use std::error::Error;

pub struct TwitterDriver {
    token: Token,
}

impl TwitterDriver {
    pub fn new(consumer_key: &str, consumer_secret: &str, access_key: &str, access_secret: &str) -> Self {
        let token = Token::Access {
            consumer: egg_mode::KeyPair::new(consumer_key.to_string(), consumer_secret.to_string()),
            access: egg_mode::KeyPair::new(access_key.to_string(), access_secret.to_string()),
        };
        Self { token }
    }

    pub async fn tweet_post(&self, text: &str) -> Result<(), Box<dyn Error>> {
        let draft = DraftTweet::new(text);
        draft.send(&self.token).await?;
        Ok(())
    }
}
