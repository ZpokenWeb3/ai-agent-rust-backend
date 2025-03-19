use rig::providers::openai;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
struct Person {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub job: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let openai_client = openai::Client::from_env();

    let data_extractor = openai_client.extractor::<Person>("gpt-4").build();

    let person = data_extractor
        .extract("Hello my name is John Doe! I am a software engineer.")
        .await?;

    println!("GPT-4: {}", serde_json::to_string_pretty(&person).unwrap());

    Ok(())
}