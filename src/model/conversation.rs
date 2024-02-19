use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Conversation {
    pub messages: Vec<message>
}

impl Conversation {
    pub fn new() -> Conversation {
        Conversation {
            messages: Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub userMsg: bool,
    pub text: String,
}