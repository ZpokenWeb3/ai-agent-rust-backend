/// Usage example
// mod price_forecasting;
// use price_forecasting::run_price_forecast;
// #[tokio::main]
// async fn main() {
//     let historical_price_data = "BTC: 50000, 51000, 49500, 50500";
//     let supporting_context = "Bitcoin is experiencing high volatility.";
//     let forecast_horizon = "24 hours ahead";
//     match run_price_forecast(historical_price_data, supporting_context, forecast_horizon).await {
//         Ok(response) => println!("Forecast: {:?}", response),
//         Err(err) => println!("Error: {:?}", err),
//     }
// }


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
use crate::price_forecasting::models::PriceForecastResponse;

/// Initializes the LLM model for price forecasting.
pub fn init_price_forecast_model() -> Chain {
    let system_prompt_path = PathBuf::from("src/price_forecasting/prompts/price_forecasting_system_prompt.md");
    let user_prompt_path = PathBuf::from("src/price_forecasting/prompts/price_forecasting_user_prompt.md");

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

    LLMChainBuilder::new()
        .prompt(prompt)
        .llm(open_ai)
        .build()
        .expect("Failed to initialize LLM chain.")
}

/// Runs the price forecast using historical data.
pub async fn run_price_forecast(
    historical_price_data: &str,
    supporting_context: &str,
    forecast_horizon: &str,
) -> Result<PriceForecastResponse, String> {
    let chain = init_price_forecast_model();

    match chain
        .invoke(prompt_args! {
            "supporting_context" => supporting_context,
            "historical_price_data" => historical_price_data,
            "forecast_horizon" => forecast_horizon
        })
        .await
    {
        Ok(result) => {
            let parsed: Result<PriceForecastResponse, _> = serde_json::from_str(&result);
            match parsed {
                Ok(response) => Ok(response),
                Err(_) => Err("Failed to parse response".to_string()),
            }
        }
        Err(e) => Err(format!("Error invoking LLM: {:?}", e)),
    }
}
