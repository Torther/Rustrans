// 管理后台处理器模块

use actix_web::{get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::config::Config;
use std::sync::{Arc, RwLock};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigUpdate {
    pub llm_api_key: Option<String>,
    pub llm_api_url: Option<String>,
    pub llm_model: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ConfigResponse {
    pub llm_api_url: String,
    pub llm_model: String,
    pub llm_api_key_masked: String,
}

/// 获取当前配置（API Key 会被脱敏）
#[get("/admin/config")]
pub async fn get_config(config: web::Data<Arc<RwLock<Config>>>) -> HttpResponse {
    let config = config.read().unwrap();
    
    let response = ConfigResponse {
        llm_api_url: config.api_url().to_string(),
        llm_model: config.model().to_string(),
        llm_api_key_masked: mask_api_key(config.api_key()),
    };
    
    HttpResponse::Ok().json(response)
}

/// 更新配置
#[post("/admin/config")]
pub async fn update_config(
    update: web::Json<ConfigUpdate>,
    config: web::Data<Arc<RwLock<Config>>>,
) -> HttpResponse {
    let mut config = config.write().unwrap();
    
    let mut updated_fields = Vec::new();
    
    // 更新 API Key（如果提供）
    if let Some(api_key) = &update.llm_api_key {
        if !api_key.is_empty() && api_key != "your-api-key-here" {
            config.llm_api_key = api_key.clone();
            updated_fields.push("API Key");
        }
    }
    
    // 更新 API URL（如果提供）
    if let Some(api_url) = &update.llm_api_url {
        if !api_url.is_empty() {
            config.llm_api_url = api_url.clone();
            updated_fields.push("API URL");
        }
    }
    
    // 更新模型（如果提供）
    if let Some(model) = &update.llm_model {
        if !model.is_empty() {
            config.llm_model = model.clone();
            updated_fields.push("模型");
        }
    }
    
    if !updated_fields.is_empty() {
        // 保存配置到文件
        if let Err(e) = config.save_to_file("config.json") {
            log::error!("保存配置失败: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "success": false,
                "message": format!("保存配置失败: {}", e)
            }));
        }
        
        let updated_items = updated_fields.join("、");
        log::info!("配置已更新: {}", updated_items);
        HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": format!("配置已更新并保存（{}）", updated_items)
        }))
    } else {
        HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "message": "没有有效的更新内容"
        }))
    }
}

/// 管理后台首页
#[get("/admin")]
pub async fn admin_index() -> HttpResponse {
    let html = include_str!("../static/admin.html");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

/// 脱敏 API Key
fn mask_api_key(key: &str) -> String {
    if key.len() <= 8 {
        return "***".to_string();
    }
    format!("{}...{}", &key[..4], &key[key.len()-4..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_api_key() {
        assert_eq!(mask_api_key("sk-1234567890abcdef"), "sk-1...cdef");
        assert_eq!(mask_api_key("short"), "***");
    }
}
