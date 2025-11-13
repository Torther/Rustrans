// HTTP 处理器模块

use actix_web::{post, web, HttpResponse};
use crate::models::{TranslateRequest, TranslateResponse};
use crate::config::Config;
use crate::language::select_target_language;
use crate::translator::{translate_with_llm, process_translation_result};
use std::sync::{Arc, RwLock};

/// 翻译接口处理器
#[post("/translate")]
pub async fn translate(
    req: web::Json<TranslateRequest>,
    config: web::Data<Arc<RwLock<Config>>>,
) -> HttpResponse {
    // 读取配置并检查是否已配置
    let config_guard = config.read().unwrap();
    if !config_guard.is_configured() {
        return HttpResponse::ServiceUnavailable().json(serde_json::json!({
            "error": "服务未配置，请先访问 /admin 配置 LLM 参数"
        }));
    }
    
    // 选择源语种和目标语种
    let (from_lang, to_lang) = select_target_language(
        &req.text,
        &req.destination,
        req.source.as_deref(),
    );
    
    // 调用大模型翻译
    match translate_with_llm(&config_guard, &req.text, &from_lang, &to_lang).await {
        Ok(translated) => {
            let result = process_translation_result(&req.text, &translated);
            
            let response = TranslateResponse {
                text: req.text.clone(),
                from: from_lang,
                to: to_lang,
                tts_uri: None,
                link: None,
                phonetic: None,
                dict: None,
                result: Some(result),
            };
            
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            log::error!("翻译失败: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("翻译失败: {}", e)
            }))
        }
    }
}
