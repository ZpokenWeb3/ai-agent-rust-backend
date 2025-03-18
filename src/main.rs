// use llm_server::core::config::{
//     Config, RedisSettings, DBSettings, TelegramSettings, TwitterSettings, SolanaSettings, SMCSettings
// };
// use llm_server::utils::{
//     redis::RedisClient, telegram_bot::TelegramBot, smc_driver::SMCDriver, twitter_driver::TwitterDriver, solana_driver::SolanaDriver
// };
// use rig::{
//     completion::Prompt,
//     embeddings::{EmbeddingsBuilder, Embed},
//     providers::openai::{Client, TEXT_EMBEDDING_ADA_002},
//     vector_store::in_memory_store::InMemoryVectorStore,
// };
// use anyhow::Result;
// use sqlx::{PgPool, postgres::PgPoolOptions}; 
// use std::sync::Arc;
// use tokio::sync::Mutex;
// use std::env;
// use serde::{Deserialize, Serialize};
// use schemars::JsonSchema;

// // ✅ Manually Implement `Embed` Trait
// #[derive(Serialize, Clone, Debug, Eq, PartialEq, Default)]
// struct Document {
//     id: String,
//     title: String,
//     content: Vec<String>,
// }

// // ✅ Implement `Embed` Trait Manually
// impl Embed for Document {
//     fn fields_to_embed(&self) -> Vec<&str> {
//         self.content.iter().map(|s| s.as_str()).collect()
//     }
// }

// #[derive(Debug, Clone)]
// pub struct RAG {
//     pub agent: Client,
// }

// impl RAG {
//     pub async fn new(api_key: &str, documents: Vec<Document>) -> Result<Self> {
//         let agent = Client::new(api_key);
//         let embedding_model = agent.embedding_model(TEXT_EMBEDDING_ADA_002);

//         // ✅ Generate embeddings
//         let embeddings = EmbeddingsBuilder::new(embedding_model.clone())
//             .documents(documents)?
//             .build()
//             .await?;

//         // ✅ Create Vector Store
//         let vector_store = InMemoryVectorStore::from_documents(embeddings);

//         // ✅ Create Index
//         let index = vector_store.index(embedding_model);

//         let rag_agent = agent.agent("gpt-4")
//             .preamble("
//                 You are a helpful assistant that retrieves knowledge from an external document store.
//                 Answer the user's query based on the most relevant retrieved information.
//             ")
//             .dynamic_context(1, index) // ✅ Enables Retrieval-Augmented Generation
//             .build();

//         Ok(Self { agent: rag_agent })
//     }

//     /// ✅ Processes a query with retrieval
//     pub async fn query(&self, prompt: &str) -> Result<String> {
//         let response = self.agent.prompt(prompt).await?;
//         Ok(response)
//     }
// }

// #[tokio::main]
// async fn main() -> Result<()> {
//     dotenv::dotenv().ok();

//     let config = Config::new();
//     let redis_settings = RedisSettings::new();
//     let db_settings = DBSettings::new();
//     let tg_settings = TelegramSettings::new();
//     let twitter_settings = TwitterSettings::new();
//     let solana_settings = SolanaSettings::new();
//     let smc_settings = SMCSettings::new();

//     // ✅ Fix Redis Client Error Handling
//     let redis_client = Arc::new(Mutex::new(
//         RedisClient::new(&redis_settings.redis_url).await
//             .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?
//     ));
//     println!("✅ Connected to Redis.");

//     let openai_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");

//     // ✅ Fix PostgreSQL Connection
//     let db_url = format!(
//         "postgres://{}:{}@{}:{}/{}",
//         db_settings.db_user, db_settings.db_pw, db_settings.db_host, db_settings.db_port, db_settings.db_name
//     );

//     let db_pool = PgPoolOptions::new()
//         .max_connections(5)
//         .connect(&db_url)
//         .await?;
//     println!("✅ Connected to PostgreSQL Database.");

//     // ✅ Initialize RAG with Document Store
//     let documents = vec![
//         Document {
//             id: "doc1".to_string(),
//             title: "Rust Programming".to_string(),
//             content: vec!["Rust is a modern systems programming language focusing on safety and concurrency.".to_string()],
//         },
//         Document {
//             id: "doc2".to_string(),
//             title: "Zero Knowledge Proofs".to_string(),
//             content: vec!["ZKPs allow one party to prove knowledge of information without revealing it.".to_string()],
//         }
//     ];

//     let rag = RAG::new(&openai_api_key, documents).await?;
//     println!("✅ RAG system initialized.");

//     // ✅ Telegram Bot
//     let tg_bot = TelegramBot::new(
//         &tg_settings.tg_token,
//         &tg_settings.tg_chat_id,
//     );
//     println!("✅ Telegram Bot initialized.");

//     // ✅ Smart Contract Driver (Fix `.await`)
//     let smc_driver = SMCDriver::new(
//         &smc_settings.web3_provider,
//         &smc_settings.prize_pool_contract_address,
//         &smc_settings.bonding_contract_address,
//         &smc_settings.owner_private_key,
//     ).await?; // **Fix `.await`**
//     println!("✅ Web3 Smart Contract Driver initialized.");

//     // ✅ Twitter Driver
//     let twitter_driver = TwitterDriver::new(
//         &twitter_settings.api_key,
//         &twitter_settings.api_secret,
//         &twitter_settings.access_token,
//         &twitter_settings.access_secret,
//     );
//     println!("✅ Twitter Driver initialized.");

//     // ✅ Solana Driver
//     let agent_keypair = solana_settings.agent_keypair.unwrap_or("default_keypair.json".to_string());
//     let solana_driver = SolanaDriver::new(
//         &solana_settings.rpc_url,
//         agent_keypair,
//     );
//     println!("✅ Solana Driver initialized.");

//     // ✅ Use RAG Directly in Main
//     let response = rag.query("What is Zero Knowledge Proof?").await?;
//     println!("🤖 AI Response: {}", response);

//     Ok(())
// }

use rig::providers::openai;
use rig::vector_store::in_memory_store::InMemoryVectorStore;
use rig::vector_store::VectorStore;
use rig::embeddings::EmbeddingsBuilder;
use rig::cli_chatbot::cli_chatbot;  
use std::path::Path;
use anyhow::{Result, Context};
use pdf_extract::extract_text;
 
fn load_pdf_content<P: AsRef<Path>>(file_path: P) -> Result<String> {
    extract_text(file_path.as_ref())
        .with_context(|| format!("Failed to extract text from PDF: {:?}", file_path.as_ref()))
}
 
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize OpenAI client
    let openai_client = openai::Client::from_env();
    let embedding_model = openai_client.embedding_model("text-embedding-ada-002");
 
    // Create vector store
    let mut vector_store = InMemoryVectorStore::default();
 
    // Get the current directory and construct paths to PDF files
    let current_dir = std::env::current_dir()?;
    let documents_dir = current_dir.join("documents");
 
    let pdf1_path = documents_dir.join("Moores_Law_for_Everything.pdf");
    let pdf2_path = documents_dir.join("The_Last_Question.pdf");
 
    // Load PDF documents
    let pdf1_content = load_pdf_content(&pdf1_path)?;
    let pdf2_content = load_pdf_content(&pdf2_path)?;
 
    // Create embeddings and add to vector store
    let embeddings = EmbeddingsBuilder::new(embedding_model.clone())
        .simple_document("Moores_Law_for_Everything", &pdf1_content)
        .simple_document("The_Last_Question", &pdf2_content)
        .build()
        .await?;
 
    vector_store.add_documents(embeddings).await?;
 
    // Create RAG agent
    let rag_agent = openai_client.context_rag_agent("gpt-3.5-turbo")
        .preamble("You are a helpful assistant that answers questions based on the given context from PDF documents.")
        .dynamic_context(2, vector_store.index(embedding_model))
        .build();
 
    // Use the cli_chatbot function to create the CLI interface
    cli_chatbot(rag_agent).await?;
 
    Ok(())
}