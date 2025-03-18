use dotenv::dotenv;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub api_v1_prefix: String,
    pub app_domain: String,
    pub web3_provider: String,
    pub openai_api_key: String,
}

impl Config {
    pub fn new_config() -> Self {
        dotenv().ok();
        Self {
            api_v1_prefix: "/api/v1".to_string(),
            app_domain: "api.agent.zpoken.dev".to_string(),
            web3_provider: "https://1rpc.io/sepolia".to_string(),
            openai_api_key: env::var("OPENAI_API_KEY").unwrap_or_default(),
        }
    }
}

#[derive(Debug)]
pub struct RedisSettings {
    pub redis_host: String,
    pub redis_port: String,
    pub redis_user: String,
    pub redis_password: String,
    pub redis_url: String,
}

impl RedisSettings {
    pub fn new_redis() -> Self {
        dotenv().ok();
        let redis_host = env::var("REDIS_HOST").unwrap_or_default();
        let redis_port = env::var("REDIS_PORT").unwrap_or_default();
        let redis_user = env::var("REDIS_USER").unwrap_or_default();
        let redis_password = env::var("REDIS_PASSWORD").unwrap_or_default();
        let redis_url = format!("redis://{}:{}@{}:{}", redis_user, redis_password, redis_host, redis_port);
        
        println!("Redis URL: {}", redis_url); // Debug print

        Self {
            redis_host,
            redis_port,
            redis_user,
            redis_password,
            redis_url,
        }
    }
}

#[derive(Debug)]
pub struct TelegramSettings {
    pub tg_token: String,
    pub tg_chat_id: String,
}

impl TelegramSettings {
    pub fn new_telegram() -> Self {
        dotenv().ok();
        Self {
            tg_token: env::var("TG_TOKEN").unwrap_or_default(),
            tg_chat_id: env::var("TG_CHAT_ID").unwrap_or_default(),
        }
    }
}

#[derive(Debug)]
pub struct DBSettings {
    pub db_name: String,
    pub db_user: String,
    pub db_host: String,
    pub db_port: String,
    pub db_pw: String,
    pub sqlalchemy_database_url: String,
    pub db_echo: bool,
}

impl DBSettings {
    pub fn new_db() -> Self {
        dotenv().ok();
        let db_name = env::var("DB_NAME").unwrap_or_default();
        let db_user = env::var("DB_USER").unwrap_or_default();
        let db_host = env::var("DB_HOST").unwrap_or_default();
        let db_port = env::var("DB_PORT").unwrap_or_default();
        let db_pw = env::var("DB_PW").unwrap_or_default();
        let sqlalchemy_database_url = format!(
            "postgresql+asyncpg://{}:{}@{}:{}/{}",
            db_user, db_pw, db_host, db_port, db_name
        );

        Self {
            db_name,
            db_user,
            db_host,
            db_port,
            db_pw,
            sqlalchemy_database_url,
            db_echo: false,
        }
    }
}

#[derive(Debug)]
pub struct SMCSettings {
    pub web3_provider: String,
    pub prize_pool_contract_address: String,
    pub bonding_contract_address: String,
    pub owner_private_key: String,
    pub prize_pool_admin_private_key: String,
}

impl SMCSettings {
    pub fn new_smc() -> Self {
        dotenv().ok();
        Self {
            web3_provider: "https://1rpc.io/sepolia".to_string(),
            prize_pool_contract_address: env::var("PRIZE_POOL_CONTRACT_ADDRESS").unwrap_or_default(),
            bonding_contract_address: env::var("BONDING_CONTRACT_ADDRESS").unwrap_or_default(),
            owner_private_key: env::var("OWNER_PRIVATE_KEY").unwrap_or_default(),
            prize_pool_admin_private_key: env::var("PRIZE_POOL_ADMIN_PRIVATE_KEY").unwrap_or_default(),
        }
    }
}

#[derive(Debug)]
pub struct SolanaSettings {
    pub rpc_url: String,
    pub solana_config_path: String,
    pub agent_keypair: String,
}

impl SolanaSettings {
    pub fn new_solana() -> Self {
        dotenv().ok();
        Self {
            rpc_url: env::var("RPC_URL").unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string()),
            solana_config_path: env::var("SOLANA_CONFIG_PATH").unwrap_or_else(|_| "configs/mainnet_raydium.json".to_string()),
            agent_keypair: env::var("AGENT_KEYPAIR").unwrap_or_default(),
        }
    }
}

#[derive(Debug)]
pub struct TwitterSettings {
    pub api_key: String,
    pub api_secret: String,
    pub access_token: String,
    pub access_secret: String,
    pub bearer_token: String,
}

impl TwitterSettings {
    pub fn new_twitter() -> Self {
        dotenv().ok();
        Self {
            api_key: env::var("API_KEY").unwrap_or_default(),
            api_secret: env::var("API_SECRET").unwrap_or_default(),
            access_token: env::var("ACCESS_TOKEN").unwrap_or_default(),
            access_secret: env::var("ACCESS_SECRET").unwrap_or_default(),
            bearer_token: env::var("BEARER_TOKEN").unwrap_or_default(),
        }
    }
}
