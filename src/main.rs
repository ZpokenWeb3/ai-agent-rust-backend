use crate::config::{Config, RedisSettings, DBSettings, TelegramSettings, TwitterSettings, SolanaSettings, SMCSettings};
use crate::utils::rag::RAG;
use crate::utils::telegram_bot::TelegramBot;
use crate::utils::smc_driver::SMCDriver;
use crate::utils::twitter_driver::TwitterDriver;
use crate::utils::solana_driver::SolanaDriver;

use anyhow::Result;
use sqlx::{PgPool, postgres::PgPoolOptions}; 
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();  

    
    let config = Config::new();
    let redis_settings = RedisSettings::new();
    let db_settings = DBSettings::new();
    let tg_settings = TelegramSettings::new();
    let twitter_settings = TwitterSettings::new();
    let solana_settings = SolanaSettings::new();
    let smc_settings = SMCSettings::new();

    
    let redis_client = RedisClient::new(&redis_settings.redis_url).await?;
    println!("✅ Connected to Redis.");

    
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_settings.sqlalchemy_database_url)
        .await?;
    println!("✅ Connected to PostgreSQL Database.");

    
    let rag = RAG::new(&config.openai_api_key).await?;
    println!("✅ RAG system initialized.");

    
    let tg_bot = TelegramBot::new(&tg_settings.tg_token, &tg_settings.tg_chat_id);
    println!("✅ Telegram Bot initialized.");

    
    let smc_driver = SMCDriver::new(
        &smc_settings.web3_provider,
        &smc_settings.prize_pool_contract_address,
        &smc_settings.owner_private_key,
        &smc_settings.bonding_contract_address,
    );
    println!("✅ Web3 Smart Contract Driver initialized.");

    
    let twitter_driver = TwitterDriver::new(
        &twitter_settings.bearer_token,
        &twitter_settings.api_key,
        &twitter_settings.api_secret,
        &twitter_settings.access_token,
    );
    println!("✅ Twitter Driver initialized.");

    
    let solana_driver = SolanaDriver::new(
        &solana_settings.rpc_url,
        &solana_settings.agent_keypair,
    );
    println!("✅ Solana Driver initialized.");

    
    let response = rag.agent.prompt("Tell me a joke.").await?;
    println!("🤖 AI Response: {}", response);

    Ok(())
}
