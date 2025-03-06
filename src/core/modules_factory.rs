use redis::aio::ConnectionManager;
use redis::Client;
use openai::Client as OpenAIClient;
use tokio;
use crate::config::*;
use crate::utils::telegram_bot::TelegramBot;
use crate::utils::smc_driver::SMCDriver;
use crate::utils::twitter_driver::TwitterDriver;
use crate::utils::solana_driver::SolanaDriver;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let redis_config = RedisSettings::new();
    let redis_client = Client::open(redis_config.redis_url.clone())?;
    let redis_db = ConnectionManager::new(redis_client).await?;

    let openai_config = Config::new();
    let openai_client = OpenAIClient::new(openai_config.openai_api_key);

    let tg_conf = TelegramSettings::new();
    let tg_bot = TelegramBot::new(tg_conf.tg_token, tg_conf.tg_chat_id);

    let smc_config = SMCSettings::new();
    let smc_driver = SMCDriver::new(
        smc_config.web3_provider,
        smc_config.prize_pool_contract_address,
        smc_config.owner_private_key,
        smc_config.bonding_contract_address,
        smc_config.prize_pool_admin_private_key,
    );

    let twitter_settings = TwitterSettings::new();
    let twitter_driver = TwitterDriver::new(
        twitter_settings.bearer_token,
        twitter_settings.api_key,
        twitter_settings.api_secret,
        twitter_settings.access_token,
        twitter_settings.access_secret,
    );

    let solana_settings = SolanaSettings::new();
    let solana_driver = SolanaDriver::new(
        solana_settings.rpc_url,
        solana_settings.solana_config_path,
        solana_settings.agent_keypair,
    );

    Ok(())
}
