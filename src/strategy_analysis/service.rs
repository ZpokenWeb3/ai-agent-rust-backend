use std::fs;
use std::path::PathBuf;
use langchain_rust::{
    chain::{Chain, LLMChainBuilder},
    fmt_message, fmt_template,
    language_models::llm::LLM,
    llm::openai::{OpenAI, OpenAIModel},
    message_formatter,
    prompt::HumanMessagePromptTemplate,
    prompt_args,
    schemas::messages::Message,
    template_fstring,
};
use crate::strategy_analysis::models::StrategyAnalysis;
use crate::strategy_analysis::strategy::Strategy;

/// Initializes the AI-powered strategy analysis model.
pub fn init_strategy_analysis_model() -> (impl Chain, Strategy) {
    let system_prompt_path = PathBuf::from("src/strategy_analysis/prompts/system_prompt.md");
    let user_prompt_path = PathBuf::from("src/strategy_analysis/prompts/user_prompt.md");

    let system_prompt = fs::read_to_string(&system_prompt_path)
        .expect("Failed to read system prompt file.");
    let user_prompt = fs::read_to_string(&user_prompt_path)
        .expect("Failed to read user prompt file.");

    let open_ai = OpenAI::default().with_model(OpenAIModel::Gpt4oMini.to_string());

    let prompt = message_formatter![
        fmt_message!(Message::new_system_message(system_prompt)),
        fmt_template!(HumanMessagePromptTemplate::new(template_fstring!(
            "{input}", "input"
        )))
    ];

    let llm_chain = LLMChainBuilder::new()
        .prompt(prompt)
        .llm(open_ai)
        .build()
        .expect("Failed to initialize LLM chain.");

    let strategy = Strategy::from_file("momentum_strategy_config.md");

    (llm_chain, strategy)
}

// usage example
// mod strategy_analysis;

// use strategy_analysis::init_strategy_analysis_model;

// #[tokio::main]
// async fn main() {
//     let (llm_chain, strategy) = init_strategy_analysis_model();
    
//     println!("Strategy Rules: {}", strategy.rules);
    
//     // Now, you can invoke llm_chain for analysis
// }


