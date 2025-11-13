// HTTP 处理器模块

use actix_web::{post, web, HttpResponse};
use validator::Validate;
use crate::models::{TranslateRequest, TranslateResponse};
use crate::config::Config;
use crate::language::select_target_language;
use crate::translator::{translate_with_llm, process_translation_result};
use crate::error::AppError;
use crate::health;
use std::sync::Arc;

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
    req.validate()
        .map_err(|e| AppError::Validation(format!("输入验证失败: {}", e)))?;

    // 检查配置是否已配置
    let is_configured = {
        let config_guard = config.read();
        config_guard.is_configured()
    };

    if !is_configured {
        app_state.record_error();
        return Err(AppError::ServiceNotConfigured);
    }

    // 选择源语种和目标语种
    let (from_lang, to_lang) = select_target_language(
        &req.text,
        &req.destination,
        req.source.as_deref(),
    );

    // 克隆必要的配置数据，避免在 await 点持有锁
    let (api_url, api_key, model) = {
        let config_guard = config.read();
        (
            config_guard.api_url().to_string(),
            config_guard.api_key().to_string(),
            config_guard.model().to_string()
        )
    };

    // 调用大模型翻译
    let config_clone = Config::new(api_url, api_key, model);
    match translate_with_llm(&config_clone, &req.text, &from_lang, &to_lang).await {
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

            app_state.record_success(start_time.elapsed());
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            app_state.record_error();
            Err(AppError::Translation(e.to_string()))
        }
    }
}
