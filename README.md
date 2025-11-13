# Rustrans

基于 LLM 的翻译服务，使用 Rust 和 Actix-web 构建。

## 功能特性

- 高性能 Rust Web 服务
- 支持 OpenAI 兼容 API
- 自动语言检测
- 智能文本格式化（pangu 排版优化）
- Web 配置管理界面
- Docker 容器化部署

## 快速开始

### Docker 部署

```bash
docker-compose up -d
```

访问管理界面配置 LLM：http://localhost:9999/admin

### 本地开发

```bash
# 构建
cargo build

# 运行（默认端口 9999）
cargo run

# 指定端口
cargo run -- --port 8080
```

## API

### 翻译

```
POST /translate
Content-Type: application/json
```

```json
{
  "text": "Hello, world!",
  "destination": ["zh-CN"],
  "source": "en"
}
```

### 配置管理

- `GET /admin` - 管理界面
- `GET /admin/config` - 获取配置
- `POST /admin/config` - 更新配置

## 配置

### 环境变量

- `PORT` - 服务端口（默认 9999）
- `RUST_LOG` - 日志级别

### 配置文件

`config.json`

```json
{
  "llm_api_key": "your-api-key",
  "llm_api_url": "https://api.example.com/v1/chat/completions",
  "llm_model": "gpt-4o-mini"
}
```

## 技术栈

- Actix-web - Web 框架
- Reqwest - HTTP 客户端
- Serde - 序列化
- Tokio - 异步运行时
- Whichlang - 语言检测
- Pangu - 排版优化

## 许可证

MIT License
