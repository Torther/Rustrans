mod models;
mod language;
mod translator;
mod handlers;
mod config;
mod admin;
mod error;
mod health;

use actix_web::{App, HttpServer, web, middleware};
use actix_cors::Cors;
use config::Config;
use handlers::translate;
use admin::{admin_index, get_config, update_config};
use health::{health_check, metrics, llm_health_check, AppState};
use std::sync::Arc;
use std::env;
use clap::Parser;

/// 翻译服务
#[derive(Parser, Debug)]
#[command(name = "Rustrans")]
#[command(about = "基于 LLM 的翻译服务", long_about = None)]
struct Args {
    /// 服务端口
    #[arg(short, long)]
    port: Option<u16>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 解析命令行参数
    let args = Args::parse();
    
    // 加载 .env 文件（可选）
    dotenv::dotenv().ok();
    
    // 初始化日志，使用自定义格式
    env_logger::Builder::from_env(env_logger::Env::new().default_filter_or("info"))
        .format(|buf, record| {
            use std::io::Write;
            writeln!(
                buf,
                "[{} {}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();
    
    // 端口优先级: 命令行参数 > 环境变量 > 默认值(9999)
    let port = if let Some(p) = args.port {
        p
    } else if let Ok(port_str) = env::var("PORT") {
        port_str.parse::<u16>().unwrap_or(9999)
    } else {
        9999
    };

    // 加载配置
    let config = match Config::load() {
        Ok(cfg) => {
            if !cfg.is_configured() {
                log::warn!("LLM 未配置，请访问 http://127.0.0.1:{}/admin 进行配置", port);
            }
            cfg
        }
        Err(e) => {
            log::error!("加载配置失败: {}", e);
            Config::default()
        }
    };
    
    // 使用 Arc<RwLock> 包装配置，使其可以在运行时修改
    let shared_config = Arc::new(parking_lot::RwLock::new(config));

    // 创建应用状态用于监控
    let app_state = AppState::new();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(shared_config.clone()))
            .app_data(web::Data::new(app_state.clone()))
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(middleware::NormalizePath::trim())
            .service(translate)
            .service(admin_index)
            .service(get_config)
            .service(update_config)
            .service(health_check)
            .service(metrics)
            .service(llm_health_check)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
