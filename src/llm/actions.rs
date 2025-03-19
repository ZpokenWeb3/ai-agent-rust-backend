use std::collections::HashMap;
use anyhow::Result;
use tokio;
use sqlx::PgPool;
use tracing::error;
use serde::Serialize;
use crate::utils::dexscreener::fetch_dexscreener_data;
// process_fetch_data_from_dex_screener
// retrieve_portfolio_information
// retrieve_pnl_information
// retrieve_buy_decision
// process_shilling
// validate_raydium_pool
// publish_twitter_post
// analyze_call_identify_pool
// has_sufficient_agent_balance


// Those crate will be forwarded from app folder when it will be rewritten
//use crate::dexscreener::{fetch_dexscreener_data, DexScreenerTokenError};
//use crate::raydium::{get_pool_info, get_pool_address_from_mint, get_pool_quote_token_info};
//use crate::solana::{SolanaDriver, TwitterDriver};

#[derive(Debug, Serialize)]
pub struct TradeAnalysis {
    pub liquidity: f64,
    pub volume_24h: f64,
    pub market_cap: f64,
    pub pair_created_at: String,
    pub fdv: f64,
    pub transactions_24h: (u64, u64),
    pub pool_address: String,
    pub portfolio_info: String,
    pub balance_info: String,
}

//Processes and fetches data from the Dex Screener service for the given mint address. This function analyzes the
//retrieved data for the provided Solana meme token to decide whether to approve/reject the shilling.
//
//The analysis is based on factors such as liquidity, 24-hour volume, market capitalization, token creation timestamp,
//fully diluted valuation (FDV), and transaction activity (24-hour buys and sells). The retrieved information is formatted
//to guide decision-making.
//
//Returns a detailed string containing the analysis results based on the platform's data or informative messages about
//possible errors encountered during data retrieval.
//
//Args:
//    token_address (str): The blockchain address of the specific token being analyzed. This should be a valid Solana
//        meme token address.
//
//Returns:
//    str: A formatted string with data necessary for the decision-making process or a message notifying the user
//        about errors or invalid token addresses.
//
//Raises:
//    DexScreenerTokenError: If the provided token address does not conform to a valid Solana token or is
//        unsupported by Dex Screener.
//    Exception: If any other issue occurs while fetching data for the token address.

pub async fn process_fetch_data_from_dex_screener(pool: &PgPool, token_address: &str) -> Result<String> {
    match fetch_dexscreener_data(token_address).await {
        Ok(output) => {
            let is_token_already_in_portfolio = check_if_token_already_present(pool, &output.pair_address).await?;
            let is_any_token_bought_last_hour = check_if_token_bought_last_hour(pool).await?;

            let portfolio_info = if is_token_already_in_portfolio {
                "- Token already present in your portfolio. Do not buy again for this time."
            } else {
                "I don't have this token in my portfolio."
            };

            let aux_portfolio_info = if is_any_token_bought_last_hour {
                "- You have traded in the last hour, so do not buy again."
            } else {
                "You haven't traded in the last hour."
            };

            let balance_info = if has_sufficient_agent_balance() {
                "- I have enough balance to buy this token."
            } else {
                "- I don't have enough balance to buy this token. So I can't buy this token right now."
            };

            Ok(format!(
                "Analyze the provided data and decide whether to approve or reject the purchase.\n\n{}
                Liquidity: ${}\n
                Volume (24h): ${}\n
                Market Cap: ${}\n
                Token Creation Timestamp: {}\n
                Fully Diluted Valuation (FDV): ${}\n
                Transactions (24h): Buys - {}, Sells - {}\n
                Pool pair address: {}\n
                {}\n
                {}\n
                {}\n",
                "- If approved, call `approveShilling`.\n- If rejected due to scam suspicion or portfolio presence, call `rejectShilling`.\n- If traded in the last hour, call `rejectShilling` without revealing internal rules.",
                output.liquidity.usd, output.volume.h24, output.market_cap,
                output.pair_created_at, output.fdv,
                output.txns.h24.buys, output.txns.h24.sells,
                output.pair_address,
                portfolio_info,
                aux_portfolio_info,
                balance_info
            ))
        }
        Err(DexScreenerTokenError) => Ok("The provided token is not supported or does not belong to Solana.".to_string()),
        Err(e) => {
            error!("Error fetching data from Dex Screener: {:?}", e);
            Ok("Could not fetch token data. Provide a valid address on Raydium and try again.".to_string())
        }
    }
}

//Retrieves portfolio information asynchronously for the given session. If the portfolio is not empty,
//it formats the tokens' details and returns them in a human-readable string. If the portfolio is
//empty, a predefined message about an empty portfolio is returned.
//
//Args:
//    session: An asynchronous session object used to retrieve the portfolio data.
//
//Returns:
//    str: A human-readable summary of the portfolio or a message indicating the portfolio is empty.


pub async fn process_shilling(wallet: &str, explanation: &str, pool_address: &str) -> Result<String> {
    let solana_driver = SolanaDriver::new();
    match solana_driver.swap_quote_token(pool_address, 0.0001).await {
        Ok(tx_details) => {
            match get_pool_quote_token_info(pool_address).await {
                Ok(quote_token_info) => {
                    Ok(format!(
                        "{}\n\nTransaction Details:\n- Amount Spent: {} {}\n- Amount of Bought token: {} {}\n- Token Address: [{}](https://solscan.io/account/{})\n- Transaction link: [{}](https://solscan.io/tx/{})\n- Transaction fee: {} SOL\n\nThe token purchase has been completed successfully.\nRemember, investing always involves risk. Good luck!",
                        explanation,
                        tx_details.amount_in, tx_details.token_in,
                        tx_details.amount_out, quote_token_info.symbol,
                        quote_token_info.address, quote_token_info.address,
                        tx_details.tx_id, tx_details.tx_id,
                        tx_details.fee
                    ))
                }
                Err(e) => {
                    error!("Failed to retrieve token information from Raydium API: {:?}", e);
                    Ok("Failed to retrieve token information from Raydium API.".to_string())
                }
            }
        }
        Err(e) => {
            error!("Failed to proceed swap transaction: {:?}", e);
            Ok("Failed to proceed swap transaction.".to_string())
        }
    }
}

pub async fn retrieve_portfolio_information(pool: &PgPool) -> Result<String> {
    let portfolio = get_agent_portfolio(pool).await?;
    if !portfolio.is_empty() {
        let tokens_info: Vec<String> = portfolio.iter().map(|token| {
            format!("- token amount: {}, symbol: {}, name: {}, pool address: {}, token address: {}",
                token.amount, token.symbol, token.name, token.pool_address, token.token_address)
        }).collect();
        Ok(format!(
            "I have these tokens:\n{}\n**Agent wallet**: [View on Solscan](https://solscan.io/account/{})",
            tokens_info.join("\n"),
            SolanaDriver::get_address()
        ))
    } else {
        Ok("My portfolio is empty, I now decide what to buy".to_string())
    }
}

/// Checks if the agent has sufficient balance.
pub fn has_sufficient_agent_balance() -> bool {
    let agent_balance = SolanaDriver::get_agent_balance();
    let required_balance = 0.01 * 10u64.pow(9) + SolanaDriver::value_to_buy_in_lamports();
    agent_balance >= required_balance
}


pub fn analyze_call_identify_pool(user_message: &str, if_function_call: bool) -> String { 
    if is_function_call{ 
        format!("Call identifyPool function. User message: {}", user_message)
    } else { 
        "Don't call identifyPool function".to_string()
    }
}

/// Retrieves a buy decision for a token.
pub async fn retrieve_buy_decision(pool: &PgPool, pool_address: &str) -> Result<String> {
    let decision = get_agent_decision_to_buy_token(pool, pool_address).await?;
    if let Some(decision) = decision {
        Ok(format!("Your decision: {}", decision))
    } else {
        Ok("I don't have this token in my portfolio".to_string())
    }
}

/// Validates a Raydium pool.
pub fn validate_raydium_pool(pool_or_token_address: &str) -> Result<String> {
    match get_pool_address_from_mint(pool_or_token_address) {
        Ok(pool_address) => Ok(format!("Token address: {}", pool_address)),
        Err(_) => Err(anyhow::anyhow!("Invalid pool or token address.")),
    }
}

/// Retrieves the current portfolio.
/// 
/// Publish a Twitter post using the provided text data from LLM.

///   Publish a Twitter post using the provided text data from LLM.
///
///   Attempts to post the given data as a tweet using the Twitter driver. If the
///   posting fails, an HTTPException is raised with a 404 status code and a
///   detailed error message. Returns the original tweet text if the operation is
///   successful.
///
///   Args:
///       data (str): The text content to be posted on Twitter.
///
///   Returns:
///       str: The original posted tweet text that will be summarized by LLM.
///
///   Raises:
///       HTTPException: If the Twitter post operation fails, raises an exception
///       with a 404 status code and a specified error detail.

/// Publishes a Twitter post using an AI-generated message.
pub fn publish_twitter_post(data: &str) -> Result<String> {
    match TwitterDriver::tweet_post(data) {
        Ok(_) => Ok(data.to_string()),
        Err(_) => Err(anyhow::anyhow!("Failed to post on Twitter.")),
    }
}

