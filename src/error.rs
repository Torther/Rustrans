// 自定义错误类型模块

use actix_web::HttpResponse;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("配置错误: {0}")]
    Config(String),

    #[error("翻译服务错误: {0}")]
    Translation(String),

    #[error("网络请求错误: {0}")]
    Network(#[from] reqwest::Error),

    #[error("JSON序列化错误: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("IO错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("语言检测失败")]
    LanguageDetection,

    #[error("输入验证错误: {0}")]
    Validation(String),

    #[error("服务未配置")]
    ServiceNotConfigured,

    #[error("请求过大")]
    RequestTooLarge,
}

pub type AppResult<T> = Result<T, AppError>;

impl actix_web::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let status = match self {
            AppError::ServiceNotConfigured => actix_web::http::StatusCode::SERVICE_UNAVAILABLE,
            AppError::Validation(_) => actix_web::http::StatusCode::BAD_REQUEST,
            AppError::RequestTooLarge => actix_web::http::StatusCode::PAYLOAD_TOO_LARGE,
            AppError::Translation(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        };

        HttpResponse::build(status).json(serde_json::json!({
            "error": self.to_string()
        }))
    }
}