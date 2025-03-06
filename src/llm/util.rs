use anyhow::{Result, anyhow};
use serde_json::{json, Value};
use sqlx::PgPool;
use tracing::{error, info};
use crate::llm_service::{
    process_fetch_data_from_dex_screener, process_shilling, retrieve_portfolio_information,
    retrieve_buy_decision, retrieve_pnl_information, validate_raydium_pool, publish_twitter_post,
    analyze_call_identify_pool
};
use crate::models::{ConversationStatus, LlmErrors};
use crate::openai_client;
use rig::providers::openai;
use async_trait::async_trait;

/// Call the appropriate function based on LLM request
pub async fn call_function(name: &str, args: &Value, pool: &PgPool) -> Result<Value> {
    match name {
        "fetch_pool_data" => process_fetch_data_from_dex_screener(args).await,
        "approveShilling" => process_shilling(args),
        "retrieveCurrentPortfolio" => retrieve_portfolio_information(args, pool).await,
        "retrieveBuyExplanation" => retrieve_buy_decision(args, pool).await,
        "retrievePnlInformation" => retrieve_pnl_information(args, pool).await,
        "identifyPool" => validate_raydium_pool(args),
        "generatePostInTwitter" => publish_twitter_post(args),
        "analyzeCallIdentifyPool" => analyze_call_identify_pool(args),
        _ => Err(anyhow!(LlmErrors::CALL_FUNCTION_ERROR)),
    }
}

/// Process response filtering
pub async fn process_filtering_reply(
    reply: Value,
    messages: &mut Vec<Value>,
    tools: Value,
    pool: &PgPool,
) -> Result<(String, ConversationStatus, Option<Value>)> {
    let filtering_reply = get_reply(messages, &tools, "auto").await?;

    if let Some(tool_calls) = filtering_reply.get("tool_calls") {
        let tool_call = &tool_calls[0];
        let (name, args) = parse_tool_call(tool_call)?;

        info!("Called filtering function: {}", name);
        messages.push(filtering_reply.clone());

        if name == "identifyPool" {
            let (result, is_pool_exists, aux_data) = call_function(&name, &args, pool).await?;
            messages.push(json!({"role": "tool", "tool_call_id": tool_call["id"], "content": result}));

            let nested_reply = get_reply(messages, &tools, "auto").await?;
            let status = if is_pool_exists { ConversationStatus::ReadyToShilling } else { ConversationStatus::Discuss };
            return Ok((nested_reply["content"].to_string(), status, aux_data));
        }
    }

    Ok((reply["content"].to_string(), ConversationStatus::Discuss, None))
}

/// Process LLM tool calls
pub async fn process_tool_calls(
    reply: Value,
    messages: &mut Vec<Value>,
    tools: Value,
    pool: &PgPool,
    user_address: &str,
) -> Result<(String, ConversationStatus, Option<Value>)> {
    let tool_call = &reply["tool_calls"][0];
    let (name, args) = parse_tool_call(tool_call)?;
    info!("Called function: {}", name);

    let result = call_function(&name, &args, pool).await?;
    messages.push(json!({"role": "tool", "tool_call_id": tool_call["id"], "content": result}));

    let nested_reply = get_reply(messages, &tools, "auto").await?;
    if let Some(nested_tool_calls) = nested_reply.get("tool_calls") {
        let nested_tool_call = &nested_tool_calls[0];
        let (nested_name, nested_args) = parse_tool_call(nested_tool_call)?;

        info!("Called nested function: {}", nested_name);

        if nested_name == "approveShilling" {
            let token_entity = call_function(&nested_name, &nested_args, pool).await?;
            return Ok((format!("{}. {}", nested_args["explanation"], result), ConversationStatus::Approve, Some(token_entity)));
        }
    }

    Ok((nested_reply["content"].to_string(), ConversationStatus::Discuss, None))
}

/// Wrapper to call chat completion
pub async fn chat_completion(
    messages: &Vec<Value>,
    tools: Option<Value>,
    tool_choice: &str,
    parallel_tool_calls: bool,
    model: &str,
) -> Result<Value> {
    let client = openai::Client::new(&std::env::var("OPENAI_API_KEY")?);
    let mut request = json!({
        "model": model,
        "messages": messages,
        "parallel_tool_calls": parallel_tool_calls
    });

    if let Some(t) = tools {
        request["tools"] = t;
    }
    if !tool_choice.is_empty() {
        request["tool_choice"] = json!(tool_choice);
    }

    let response = client.chat().completions().create("gpt-4o", request).await?;
    Ok(response)
}

/// Parse a tool call into its name and arguments
pub fn parse_tool_call(tool_call: &Value) -> Result<(String, Value)> {
    let args = serde_json::from_value(tool_call["function"]["arguments"].clone())?;
    let name = tool_call["function"]["name"].as_str().unwrap().to_string();
    Ok((name, args))
}

/// Get response from OpenAI chat completion
pub async fn get_reply(
    messages: &Vec<Value>,
    tools: &Value,
    tool_choice: &str,
) -> Result<Value> {
    let response = chat_completion(messages, Some(tools.clone()), tool_choice, false, "gpt-4o-mini").await?;
    if let Some(choice) = response.get("choices").and_then(|c| c.get(0)) {
        if let Some(message) = choice.get("message") {
            return Ok(message.clone());
        }
    }
    Err(anyhow!("No response from LLM"))
}

/// Async context manager to get DB session
pub async fn get_db_session(pool: &PgPool) -> Result<sqlx::Transaction<'_, sqlx::Postgres>> {
    let transaction = pool.begin().await?;
    Ok(transaction)
}
