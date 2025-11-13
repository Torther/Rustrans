// 自定义错误类型模块

use actix_web::HttpResponse;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("翻译服务错误: {0}")]
    Translation(String),

    #[error("网络请求错误: {0}")]
    Network(#[from] reqwest::Error),

    #[error("JSON序列化错误: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("IO错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("输入验证错误: {0}")]
    Validation(String),

    #[error("服务未配置")]
    ServiceNotConfigured,
}

pub type AppResult<T> = Result<T, AppError>;

impl actix_web::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let status = match self {
            AppError::ServiceNotConfigured => actix_web::http::StatusCode::SERVICE_UNAVAILABLE,
            AppError::Validation(_) => actix_web::http::StatusCode::BAD_REQUEST,
            AppError::Translation(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        };

        HttpResponse::build(status).json(serde_json::json!({
            "error": self.to_string()
        }))
    }
}
