use sqlx::PgPool;
use rig::{completion::Prompt, providers::openai};
use serde_json::json;
use tracing::{error, info};
use anyhow::Result;
use crate::models::{Trade, LlmResponse, ActionParameter, ConversationStatus};

pub async fn answer_users_msg(
    pool: &PgPool,
    message: &str,
    user_address: &str,
    history_uuid: &str,
    is_shilling_allowed: bool,

    //Get response from the LLM and process the user's message.
    //param msg: User's message.
    //param user_address: User's wallet address.
    //param history_uiid: Identifier for the message history in Redis.
    //return: A tuple containing the response message, conversation status, and (optionally) a token entity.


) -> Result<LlmResponse> {
    let openai_api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let client = openai::Client::new(&openai_api_key);

    let history_messages = get_redis_history(history_uuid).await.unwrap_or_default();
    let aux_prompt_action = if is_shilling_allowed { "shilling_allowed" } else { "shilling_not_allowed" };

    let messages = vec![
        json!({"role": "system", "content": main_prompts::get(ActionParameter::Shilling)}),
        json!({"role": "system", "content": prompt_actions::get(aux_prompt_action)}),
    ]
    .into_iter()
    .chain(history_messages.iter().map(|m| json!(m)))
    .chain(vec![json!({"role": "user", "content": message})])
    .collect::<Vec<_>>();

    let tools = if is_shilling_allowed {
        main_tools::get(ActionParameter::Shilling)
    } else {
        main_tools::get("shilling_not_allowed")
    };

    let tool_choice = if is_shilling_allowed { "required" } else { "auto" };

    match get_reply(&client, messages, tools, tool_choice).await {
        Ok(reply) => {
            if let Some(tool_calls) = reply.tool_calls {
                process_tool_calls(&client, &pool, reply, user_address).await
            } else if let Some(content) = reply.content {
                process_filtering_reply(reply, tools).await
            } else {
                Err(anyhow::anyhow!("No response from LLM"))
            }
        }
        Err(e) => {
            error!("LLM processing error: {:?}", e);
            Err(e)
        }
    }
}

// Function to generate Twitter post
pub async fn generate_twitter_post(
    pool: &PgPool,
    user_address: Option<&str>,
    trade: &Trade,
    trade_type: &str,
) -> Result<()> {
    if !["buy", "sell"].contains(&trade_type.to_lowercase().as_str()) {
        return Err(anyhow::anyhow!("trade_type must be either 'buy' or 'sell'"));
    }

    let token = get_token_by_trade_id(pool, trade.id).await?;
    let mut aux_sell_message = String::new();

    if trade_type.eq_ignore_ascii_case("sell") {
        let (pnl, percentage_pnl) = calculate_pnl(pool, trade).await?;
        aux_sell_message = format!(
            "- Got also PNL: {} SOL, how much I earned/lost in percentage: {:.2}%",
            pnl, percentage_pnl
        );
    }

    let short_solscan_link = shorten_url(&format!("https://solscan.io/tx/{}", trade.tx_id));

    let prompt_message = format!(
        "I need to publish a post with trade type: `{}`. For PNL, do not use scientific format. 
        Token ticker: {}, full name: {}
        - Amount of token: {} ({:.4} SOL)
        - Sol scan link: {}
        - User wallet address: {}
        {}",
        trade_type,
        token.symbol,
        token.name,
        trade.quote_token_quantity,
        trade.base_token_quantity,
        short_solscan_link,
        user_address.unwrap_or("Anonymous"),
        aux_sell_message
    );

    let messages = vec![
        json!({"role": "system", "content": main_prompts::get(ActionParameter::Shilling)}),
        json!({"role": "system", "content": "
                Generate a Twitter post about the trade that is under 280 characters. Write naturally as if spoken by a real person. Use minimal hashtags, but include the mandatory hashtags #KajaAI and #Raydium.
                Call function `generatePostInTwitter` to publish post in twitter.
                Do not use scientific notation for numbers (e.g., 1e-05); format them as standard decimals (e.g., 0.000).
                Also, avoid placing special characters like `@` immediately next to addresses.
                Instead of writing 'User: <address>', refer to the user as 'the user who shilled me token' and include the shortened address (shorten them to show only the first 5 and the last 5 characters, separated by an ellipsis).
                Place all hashtags on a separate line at the bottom.
                If post about buying, include a very brief and concise statement expressing your expectation from the trade.
                Always include the transaction link from SolScan in your post.
        "}),
        json!({"role": "user", "content": prompt_message}),
    ];

    let client = openai::Client::new(&std::env::var("OPENAI_API_KEY")?);
    let response = client.chat().completions().create("gpt-4o", messages).await?;

    if let Some(tool_calls) = response.tool_calls {
        for tool_call in tool_calls {
            if tool_call.function.name == "generatePostInTwitter" {
                let args = serde_json::from_str(&tool_call.function.arguments)?;
                call_function("generatePostInTwitter", args).await?;
            }
        }
    }

    Ok(())
}

// Shorten URL using TinyURL
pub fn shorten_url(url: &str) -> String {
    let api_url = format!("http://tinyurl.com/api-create.php?url={}", url);
    match reqwest::blocking::get(&api_url) {
        Ok(resp) => resp.text().unwrap_or_else(|_| url.to_string()),
        Err(_) => url.to_string(),
    }
}

// Generate selling text
pub async fn generate_selling_text(
    pool: &PgPool,
    transfer_signature: Option<&str>,
    closed_trade: &Trade,
) -> Result<String> {
    let token = get_token_by_trade_id(pool, closed_trade.id).await?;
    let (pnl, percentage_pnl) = calculate_pnl(pool, closed_trade).await?;
    let is_trade_profitable = closed_trade.profit_loss;
    let aux_message = if is_trade_profitable {
        format!(
            "I've shared with you {:.9} SOL (50% of my profit)! Check the transfer at https://solscan.io/tx/{}",
            pnl * 0.5,
            transfer_signature.unwrap_or("")
        )
    } else {
        "Unfortunately, I didn't profit from this trade, so I couldn't share any funds.".to_string()
    };

    let prompt_message = format!(
        "I sold {} {} for {:.9} SOL. My PNL is {:.9} SOL ({:.2}%). 
        Swap details: https://solscan.io/tx/{}. 
        {}",
        closed_trade.quote_token_quantity,
        token.symbol,
        closed_trade.base_token_quantity,
        pnl,
        percentage_pnl,
        closed_trade.tx_id,
        aux_message
    );

    let messages = vec![
        json!({"role": "system", "content": main_prompts::get(ActionParameter::Shilling)}),
        json!({"role": "system", "content": "
        Generate a chat message for the user about a closed selling trade. Write naturally and conversationally, as if you are a real person. 
               Ensure that you refer only to yourself—the trading agent—and do not mention any other trader names or identities. 
               Do not use scientific notation for numbers; format them as standard decimals. 
               If the trade was profitable, mention that you've shared 50% of your profit with the user by completing a transfer of the funds, and include the SolScan transaction link. 
                If the trade resulted in a loss, explain that no funds were shared. 
                Make sure the message flows naturally, reflects your own trading performance, and sounds genuine
        "}),
        json!({"role": "user", "content": prompt_message}),
    ];

    let client = openai::Client::new(&std::env::var("OPENAI_API_KEY")?);
    let response = client.chat().completions().create("gpt-4o", messages).await?;

    if let Some(content) = response.choices[0].message.content {
        Ok(content)
    } else {
        Err(anyhow::anyhow!("No response from LLM"))
    }
}
