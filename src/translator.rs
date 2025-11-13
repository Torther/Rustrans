// 大模型翻译服务模块

use crate::models::{LLMRequest, LLMResponse, Message};
use crate::config::Config;
use pangu::spacing;

/// 调用大模型进行翻译
pub async fn translate_with_llm(
    config: &Config,
    text: &str,
    from_lang: &str,
    to_lang: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let system_prompt = build_system_prompt(from_lang, to_lang);
    
    let request_body = LLMRequest {
        model: config.model().to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: system_prompt,
            },
            Message {
                role: "user".to_string(),
                content: text.to_string(),
            },
        ],
        temperature: 0.3,
    };
    
    let client = reqwest::Client::new();
    let response = client
        .post(config.api_url())
        .header("Authorization", format!("Bearer {}", config.api_key()))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;
    
    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(format!("LLM API 错误: {}", error_text).into());
    }
    
    let llm_response: LLMResponse = response.json().await?;
    
    if let Some(choice) = llm_response.choices.first() {
        let translated = choice.message.content.trim();
        // 使用 pangu 优化排版
        let formatted = spacing(translated).to_string();
        Ok(formatted)
    } else {
        Err("未收到翻译结果".into())
    }
}

/// 构建系统提示词
fn build_system_prompt(from_lang: &str, to_lang: &str) -> String {
    format!(
        "ROLE: 专业翻译专家\n\
        TASK: {from_lang} → {to_lang} 翻译\n\
        \n\
        CONSTRAINTS:\n\
        - 严格保持原文语义完整性\n\
        - 输出语言: 仅使用{to_lang}\n\
        - 格式保持: 保留所有段落和结构\n\
        - 禁止: 解释、注释、额外说明\n\
        \n\
        OUTPUT_FORMAT:\n\
        - 纯翻译文本\n\
        - 无前缀后缀\n\
        - 直接开始翻译内容\n\
        \n\
        READY."
    )
}

/// 处理翻译结果，分段并进行 pangu 格式化
pub fn process_translation_result(original: &str, translated: &str) -> Vec<String> {
    let original_paragraphs: Vec<&str> = original.split('\n').collect();
    let translated_paragraphs: Vec<&str> = translated.split('\n').collect();
    
    // 对每个段落进行 pangu 处理
    let format_paragraph = |s: &str| spacing(s).to_string();
    
    // 如果段落数量匹配，处理每个段落
    if original_paragraphs.len() == translated_paragraphs.len() {
        return translated_paragraphs
            .iter()
            .map(|s| format_paragraph(s))
            .collect();
    }
    
    // 如果不匹配，返回整体翻译结果（仍然进行 pangu 处理）
    vec![format_paragraph(translated)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_translation_result() {
        let original = "第一段\n第二段";
        let translated = "First paragraph\nSecond paragraph";
        let result = process_translation_result(original, translated);
        assert_eq!(result, vec!["First paragraph", "Second paragraph"]);
    }

    #[test]
    fn test_process_translation_result_mismatch() {
        let original = "第一段\n第二段";
        let translated = "Combined translation";
        let result = process_translation_result(original, translated);
        assert_eq!(result, vec!["Combined translation"]);
    }
    
    #[test]
    fn test_pangu_spacing() {
        let original = "第一段\n第二段";
        let translated = "这是test文本，包含English和中文混排";
        let result = process_translation_result(original, translated);
        // pangu 会在中英文之间添加空格
        assert_eq!(result, vec!["这是 test 文本，包含 English 和中文混排"]);
    }
}

