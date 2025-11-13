// 数据模型模块

use serde::{Deserialize, Serialize};

// 划词翻译请求结构
#[derive(Debug, Deserialize)]
pub struct TranslateRequest {
    #[allow(dead_code)]
    pub name: String,
    pub text: String,
    pub destination: Vec<String>,
    pub source: Option<String>,
}

// 划词翻译响应结构
#[derive(Debug, Serialize)]
pub struct TranslateResponse {
    pub text: String,
    pub from: String,
    pub to: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ttsURI")]
    pub tts_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phonetic: Option<Vec<Phonetic>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dict: Option<Vec<DictEntry>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct Phonetic {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ttsURI")]
    pub tts_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DictEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos: Option<String>,
    pub terms: Vec<String>,
}

// 大模型 API 请求结构（OpenAI 兼容格式）
#[derive(Debug, Serialize)]
pub struct LLMRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: f32,
}

#[derive(Debug, Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct LLMResponse {
    pub choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: MessageContent,
}

#[derive(Debug, Deserialize)]
pub struct MessageContent {
    pub content: String,
}
