use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct StreamMsg {
    pub id: String,
    pub model: String,
    pub choices: Vec<StreamMsgChoice>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StreamMsgChoiceDelta {
    pub content: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StreamMsgChoice {
    pub delta: StreamMsgChoiceDelta,
    pub index: u32,
    pub finish_reason: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Msg {
    pub role: String,
    pub content: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Obj {
    pub model: String,
    pub messages: Vec<Msg>,
    pub stream: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ChatCompletion {
    pub id: Option<String>,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub usage: Usage,
    pub choices: Vec<Choice>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Choice {
    pub message: Msg,
    pub finish_reason: String,
    pub index: u32,
}
