use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct LLmResponse { 
    pub text: String, 
    pub decision: String, 
    pub aux_data: Option<HashMap<String, String>>,
}

impl LLmResponse { 
    pub fn new(text: &str, decision: &str, aux_data: Option<HashMap<String, String>>) { 
        Self { 
            text: text.to_string(),
            decision: decision.to_string(),
            aux_data,
        }
    }
}