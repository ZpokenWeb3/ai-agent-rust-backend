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

/// Uses LangChain to generate a description of Alchemy API.
pub async fn describe_alchemy_api() -> String {
    let open_ai = OpenAI::default().with_model(OpenAIModel::Gpt4oMini.to_string());

    let prompt = message_formatter![
        fmt_message!(Message::new_system_message(
            "Explain the purpose and usage of the Alchemy API in a blockchain ecosystem."
        )),
        fmt_template!(HumanMessagePromptTemplate::new(template_fstring!(
            "{input}", "input"
        )))
    ];

    let chain = LLMChainBuilder::new()
        .prompt(prompt)
        .llm(open_ai.clone())
        .build()
        .unwrap();

    match chain
        .invoke(prompt_args! {
            "input" => "What is the Alchemy API and how does it help developers?"
        })
        .await
    {
        Ok(result) => result.to_string(),
        Err(e) => format!("Error invoking LangChain: {:?}", e),
    }
}
