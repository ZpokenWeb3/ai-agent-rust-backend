#[derive(Debug)]
pub enum ChatErrors {
    GoogleApiResourceExhausted,
    ChatNotFound,
    UserNotOwner,
}

impl std::fmt::Display for ChatErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ChatErrors::GoogleApiResourceExhausted => write!(f, "Google API quota limits reached"),
            ChatErrors::ChatNotFound => write!(f, "Chat not found!"),
            ChatErrors::UserNotOwner => write!(f, "That's not your chat!"),
        }
    }
}

#[derive(Debug)]
pub enum LLMErrors {
    CallFunctionError,
    TwitterPostError,
}

impl std::fmt::Display for LLMErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LLMErrors::CallFunctionError => write!(f, "Call function error!"),
            LLMErrors::TwitterPostError => write!(f, "Twitter post error!"),
        }
    }
}
