use std::env;
use tokio;

//use sqlx::PgPool;
use rig::{completion::Prompt, providers};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Create OpenAI client
    let client = providers::openai::Client::new(
        &env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set"),
    );

    // let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://user:password@localhost/db".to_string());
    // let pool = PgPool::connect(&database_url).await?;


    // mock
    // let message = "What tokens do you hold?";
    // let user_address = "ExrR3uaTmvZUV6wC733fNUBCz3FrpdRyPQYMSE5dKnQQ";
    // let history_uuid = "6b675678-04c3-488e-be5b-4ae2947b90ee";
    // let is_shilling_allowed = false;


    // Create agent with a single context prompt
    let ai_agent = client
        .agent("gpt-4o")
        .preamble("You are a comedian here to entertain the user using humour and jokes.")
        .build();

    // Prompt the agent and print the response
    let response = ai_agent.prompt("Entertain me!").await?;
    println!("{}", response);


    // match answer_users_msg(&client, &pool, message, user_address, history_uuid, is_shilling_allowed).await {
    //     Ok(response) => {
    //         info!("LLM Response: {}, Decision: {:?}", response.text, response.decision);
    //     }
    //     Err(e) => {
    //         error!("Error processing user message: {:?}", e);
    //     }
    // }


    Ok(())
}