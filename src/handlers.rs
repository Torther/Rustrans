// HTTP 处理器模块

use crate::config::Config;
use crate::error::AppError;
use crate::health;
use crate::language::select_target_language;
use crate::models::{TranslateRequest, TranslateResponse};
use crate::translator::{process_translation_result, translate_with_llm};
use actix_web::{post, web, HttpResponse};
use std::sync::Arc;
use validator::Validate;

/// 翻译接口处理器
#[post("/translate")]
pub async fn translate(
    req: web::Json<TranslateRequest>,
    config: web::Data<Arc<parking_lot::RwLock<Config>>>,
    app_state: web::Data<health::AppState>,
) -> Result<HttpResponse, AppError> {
    let start_time = std::time::Instant::now();
    app_state.record_request();
    // 验证输入数据
    if let Err(e) = req.validate() {
        let validation_response = TranslateResponse {
            text: req.text.clone(),
            from: "未知".to_string(),
            to: req.destination.first().unwrap_or(&"未知".to_string()).clone(),
            tts_uri: None,
            link: None,
            phonetic: None,
            dict: None,
            result: Some(vec![format!("输入验证失败: {}", e)]),
        };
        return Ok(HttpResponse::Ok().json(validation_response));
    }

    // 选择源语种和目标语种
    let (from_lang, to_lang) =
        select_target_language(&req.text, &req.destination, req.source.as_deref());

    // 检查配置是否已配置
    let is_configured = {
        let config_guard = config.read();
        config_guard.is_configured()
    };

    if !is_configured {
        app_state.record_error();
        let error_response = TranslateResponse {
            text: req.text.clone(),
            from: from_lang,
            to: to_lang,
            tts_uri: None,
            link: None,
            phonetic: None,
            dict: None,
            result: Some(vec!["翻译服务未配置，请访问 /admin 配置 API 后重试".to_string()]),
        };
        return Ok(HttpResponse::Ok().json(error_response));
    }

    // 克隆必要的配置数据，避免在 await 点持有锁
    let (api_url, api_key, model, system_prompt) = {
        let config_guard = config.read();
        (
            config_guard.api_url().to_string(),
            config_guard.api_key().to_string(),
            config_guard.model().to_string(),
            config_guard.system_prompt().to_string(),
        )
    };

    // 调用大模型翻译
    let config_clone = Config::new(api_url, api_key, model, system_prompt);
    let response = match translate_with_llm(&config_clone, &req.text, &from_lang, &to_lang).await {
        Ok(translated) => {
            let result = process_translation_result(&req.text, &translated);

            TranslateResponse {
                text: req.text.clone(),
                from: from_lang,
                to: to_lang,
                tts_uri: None,
                link: None,
                phonetic: None,
                dict: None,
                result: Some(result),
            }
        }
        Err(e) => {
            app_state.record_error();
            let error_message = format!("翻译失败: {}", e);

            TranslateResponse {
                text: req.text.clone(),
                from: from_lang,
                to: to_lang,
                tts_uri: None,
                link: None,
                phonetic: None,
                dict: None,
                result: Some(vec![error_message]),
            }
        }
    };

    app_state.record_success(start_time.elapsed());
    Ok(HttpResponse::Ok().json(response))
}
