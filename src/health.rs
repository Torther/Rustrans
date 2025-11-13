// 健康检查和监控模块

use actix_web::{get, web, HttpResponse};
use serde::Serialize;
use std::sync::Arc;
use std::time::{Duration, Instant};
use crate::config::Config;
use crate::error::{AppError, AppResult};

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
    pub version: String,
    pub uptime_seconds: u64,
    pub llm_configured: bool,
    pub memory_usage: MemoryUsage,
}

#[derive(Debug, Serialize)]
pub struct MemoryUsage {
    pub allocated_mb: u64,
    pub resident_mb: u64,
}

#[derive(Debug, Serialize)]
pub struct MetricsResponse {
    pub requests_total: u64,
    pub requests_success: u64,
    pub requests_error: u64,
    pub avg_response_time_ms: f64,
    pub uptime_seconds: u64,
    pub concurrent_requests: u64,
}

// 应用状态用于收集指标
#[derive(Debug, Clone)]
pub struct AppState {
    pub start_time: Instant,
    pub requests_total: Arc<std::sync::atomic::AtomicU64>,
    pub requests_success: Arc<std::sync::atomic::AtomicU64>,
    pub requests_error: Arc<std::sync::atomic::AtomicU64>,
    pub response_times: Arc<parking_lot::Mutex<Vec<Duration>>>,
    pub concurrent_requests: Arc<std::sync::atomic::AtomicU64>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            requests_total: Arc::new(std::sync::atomic::AtomicU64::new(0)),
            requests_success: Arc::new(std::sync::atomic::AtomicU64::new(0)),
            requests_error: Arc::new(std::sync::atomic::AtomicU64::new(0)),
            response_times: Arc::new(parking_lot::Mutex::new(Vec::new())),
            concurrent_requests: Arc::new(std::sync::atomic::AtomicU64::new(0)),
        }
    }

    pub fn record_request(&self) {
        self.requests_total.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.concurrent_requests.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn record_success(&self, duration: Duration) {
        self.requests_success.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.concurrent_requests.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);

        // 记录响应时间，保持最近1000个请求
        let mut times = self.response_times.lock();
        times.push(duration);
        if times.len() > 1000 {
            times.remove(0);
        }
    }

    pub fn record_error(&self) {
        self.requests_error.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.concurrent_requests.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
    }
}

/// 健康检查端点
#[get("/health")]
pub async fn health_check(
    config: web::Data<Arc<parking_lot::RwLock<Config>>>,
    app_state: web::Data<AppState>,
) -> AppResult<HttpResponse> {
    let uptime = app_state.start_time.elapsed();

    let llm_configured = {
        let config_guard = config.read();
        config_guard.is_configured()
    };

    let memory_usage = get_memory_usage();

    let response = HealthResponse {
        status: if llm_configured { "healthy" } else { "degraded" }.to_string(),
        timestamp: chrono::Local::now().to_rfc3339(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: uptime.as_secs(),
        llm_configured,
        memory_usage,
    };

    Ok(HttpResponse::Ok().json(response))
}

/// 详细指标端点
#[get("/metrics")]
pub async fn metrics(
    app_state: web::Data<AppState>,
) -> AppResult<HttpResponse> {
    let uptime = app_state.start_time.elapsed();

    let requests_total = app_state.requests_total.load(std::sync::atomic::Ordering::Relaxed);
    let requests_success = app_state.requests_success.load(std::sync::atomic::Ordering::Relaxed);
    let requests_error = app_state.requests_error.load(std::sync::atomic::Ordering::Relaxed);
    let concurrent_requests = app_state.concurrent_requests.load(std::sync::atomic::Ordering::Relaxed);

    let avg_response_time_ms = {
        let times = app_state.response_times.lock();
        if times.is_empty() {
            0.0
        } else {
            let total: Duration = times.iter().sum();
            (total.as_millis() as f64) / (times.len() as f64)
        }
    };

    let response = MetricsResponse {
        requests_total,
        requests_success,
        requests_error,
        avg_response_time_ms,
        uptime_seconds: uptime.as_secs(),
        concurrent_requests,
    };

    Ok(HttpResponse::Ok().json(response))
}

/// LLM 服务连通性检查
#[get("/health/llm")]
pub async fn llm_health_check(
    config: web::Data<Arc<parking_lot::RwLock<Config>>>,
) -> AppResult<HttpResponse> {
    let (api_url, api_key, model) = {
        let config_guard = config.read();
        if !config_guard.is_configured() {
            return Err(AppError::ServiceNotConfigured);
        }
        (
            config_guard.api_url().to_string(),
            config_guard.api_key().to_string(),
            config_guard.model().to_string()
        )
    };

    // 发送一个简单的测试请求来检查连通性
    let test_request = serde_json::json!({
        "model": model,
        "messages": [
            {
                "role": "user",
                "content": "test"
            }
        ],
        "max_tokens": 1,
        "temperature": 0.1
    });

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    let response = client
        .post(&api_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&test_request)
        .send()
        .await?;

    let status_code = response.status();
    if status_code.is_success() || status_code == 400 {
        // 400 也是可以接受的，因为我们的测试请求可能无效
        Ok(HttpResponse::Ok().json(serde_json::json!({
            "status": "healthy",
            "message": "LLM service is reachable",
            "response_code": status_code.as_u16()
        })))
    } else {
        let error_text = response.text().await.unwrap_or_default();
        Ok(HttpResponse::ServiceUnavailable().json(serde_json::json!({
            "status": "unhealthy",
            "message": "LLM service is not reachable",
            "response_code": status_code.as_u16(),
            "error": error_text
        })))
    }
}

/// 获取内存使用情况
fn get_memory_usage() -> MemoryUsage {
    // 这是一个简化的内存使用情况
    // 在生产环境中，可以使用更精确的内存监控库如 `sysinfo`
    MemoryUsage {
        allocated_mb: 0, // 实际实现中应该使用系统调用获取真实数据
        resident_mb: 0,
    }
}