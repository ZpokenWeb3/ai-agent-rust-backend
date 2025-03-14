use redis::aio::MultiplexedConnection;
use redis::Client;
use redis::aio::ConnectionManager;
use tokio;
use rig::providers::openai::Client as OpenAIClient;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signature::Signer;
use crate::core::config::*;
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
    let openai_client = OpenAIClient::new(&openai_config.openai_api_key);

    let tg_conf = TelegramSettings::new();
    let tg_bot = TelegramBot::new(&tg_conf.tg_token, &tg_conf.tg_chat_id);

    let smc_config = SMCSettings::new();
    let smc_driver = SMCDriver::new(
        &smc_config.web3_provider,
        &smc_config.prize_pool_contract_address,
        &smc_config.owner_private_key,
        &smc_config.bonding_contract_address,
    );

    let twitter_settings = TwitterSettings::new();
    let twitter_driver = TwitterDriver::new(
        &twitter_settings.bearer_token,
        &twitter_settings.api_key,
        &twitter_settings.api_secret,
        &twitter_settings.access_token,
    );

    let solana_settings = SolanaSettings::new();
    let keypair = Keypair::from_base58_string(&solana_settings.agent_keypair);
    let solana_driver = SolanaDriver::new(&solana_settings.rpc_url, keypair);

    Ok(())
}
