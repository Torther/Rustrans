# Rustrans

基于 Rust 和大语言模型（LLM）的高性能翻译服务

![CI/CD](https://github.com/Torther/rustrans/workflows/CI/CD%20Pipeline/badge.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.91.0+-orange.svg)

## ✨ 特性

- 🚀 **高性能**：基于 Rust 和 Actix-web 框架，提供极佳的性能
- 🌐 **智能翻译**：支持多种大语言模型 API（OpenAI、GLM 等）
- 🔄 **自动语言检测**：自动识别源语言
- 📝 **智能排版**：使用 Pangu 优化中英文混排格式
- 🌐 **Web 管理界面**：提供美观的管理页面进行配置
- 🐳 **Docker 支持**：提供完整的 Docker 部署方案
- 🔒 **安全可靠**：内置输入验证、错误处理和安全审计
- 📊 **监控和健康检查**：提供详细的指标和健康状态监控
- 🛡️ **企业级特性**：支持批量翻译、请求限流、连接复用

## 🚀 快速开始

### 使用 Docker（推荐）

```bash
git clone https://github.com/Torther/rustrans.git
cd rustrans
docker-compose up -d
```

访问 http://localhost:9999/admin 进行配置。

### 本地运行

1. **克隆仓库**：
```bash
git clone https://github.com/Torther/rustrans.git
cd rustrans
```

2. **安装依赖**（如果需要）：
```bash
make install-tools
```

3. **运行服务**：
```bash
make dev  # 开发模式
# 或者
cargo run -- --port 8080
```

## 📖 API 文档

### 翻译接口

```http
POST /translate
Content-Type: application/json

{
  "name": "translate",
  "text": "Hello, world!",
  "destination": ["中文(简体)", "日语"],
  "source": "英语"
}
```

### 响应示例

```json
{
  "text": "Hello, world!",
  "from": "英语",
  "to": "中文(简体)",
  "result": ["你好，世界！"]
}
```

### 健康检查

```http
GET /health
```

```json
{
  "status": "healthy",
  "timestamp": "2024-01-01T12:00:00Z",
  "version": "0.1.0",
  "uptime_seconds": 3600,
  "llm_configured": true,
  "memory_usage": {
    "allocated_mb": 128,
    "resident_mb": 64
  }
}
```

### 指标监控

```http
GET /metrics
```

```json
{
  "requests_total": 1000,
  "requests_success": 950,
  "requests_error": 50,
  "avg_response_time_ms": 245.5,
  "uptime_seconds": 3600,
  "concurrent_requests": 3
}
```

## ⚙️ 配置

配置文件 `config.json`：

```json
{
  "llm_api_key": "your-api-key",
  "llm_api_url": "https://api.openai.com/v1/chat/completions",
  "llm_model": "gpt-4o-mini"
}
```

### 环境变量

- `RUSTRANS_PORT`: 服务端口（默认：9999）
- `RUSTRANS_API_KEY`: LLM API 密钥
- `RUSTRANS_API_URL`: LLM API 地址
- `RUSTRANS_MODEL`: LLM 模型名称

## 🌍 支持的语言

- 中文（简体）
- 英语
- 日语
- 韩语
- 俄语
- 西班牙语
- 法语
- 德语
- 阿拉伯语
- 葡萄牙语
- 意大利语
- 越南语

## 🛠️ 开发

### 项目结构

```
src/
├── main.rs          # 应用入口点
├── config.rs        # 配置管理
├── models.rs        # 数据模型
├── handlers.rs      # HTTP 处理器
├── translator.rs    # 翻译服务
├── language.rs      # 语言检测
├── admin.rs         # 管理界面
├── health.rs        # 健康检查和监控
└── error.rs         # 错误处理
```

### 开发工具

```bash
# 安装开发工具
make install-tools

# 运行测试
make test

# 生成覆盖率报告
make test-coverage

# 代码检查
make lint

# 格式化代码
make format

# 安全审计
make security
```

### 使用 Makefile

项目提供了完整的 Makefile 来简化开发流程：

```bash
make help           # 查看所有可用命令
make build          # 构建项目
make test           # 运行测试
make lint           # 代码检查
make format         # 格式化代码
make clean          # 清理构建文件
make docker-build   # 构建 Docker 镜像
make docker-run     # 运行 Docker 容器
make security       # 运行安全审计
make dev            # 开发模式运行
```

## 🧪 测试

### 运行测试

```bash
# 单元测试
cargo test

# 集成测试
cargo test --test integration_test

# 性能测试
cargo bench
```

### 测试覆盖率

```bash
make test-coverage
```

## 🔒 安全

本项目采用多层安全措施：

- **输入验证**：严格的请求参数验证
- **错误处理**：结构化错误处理，避免信息泄露
- **依赖审计**：定期检查依赖漏洞
- **配置加密**：敏感信息加密存储
- **日志安全**：避免记录敏感信息

详细的安全信息请参考 [security.md](security.md)

## 🐳 Docker

### 构建 Docker 镜像

```bash
make docker-build
# 或者
docker build -t rustrans:latest .
```

### 运行 Docker 容器

```bash
make docker-run
# 或者
docker-compose up -d
```

### 生产部署

```bash
# 生产构建
make build-prod

# 创建发布包
make package
```

## 📊 监控

Rustrans 提供了完整的监控和健康检查功能：

- **健康检查**：`/health` 端点
- **指标监控**：`/metrics` 端点
- **LLM 服务检查**：`/health/llm` 端点
- **管理界面**：`/admin` 网页界面

## 🤝 贡献

我们欢迎所有形式的贡献！

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

### 开发规范

- 遵循 Rust 官方代码风格
- 所有新功能都需要测试
- 更新相关文档
- 通过所有 CI 检查

## 📄 许可证

本项目采用 MIT 许可证。详情请参阅 [LICENSE](LICENSE) 文件。

## 🙏 致谢

- [Actix-web](https://actix.rs/) - 高性能 Web 框架
- [Reqwest](https://docs.rs/reqwest/) - HTTP 客户端
- [Whichlang](https://docs.rs/whichlang/) - 语言检测
- [Pangu](https://docs.rs/pangu/) - 中英文混排格式化

## 📞 支持

如果您遇到任何问题或有任何建议，请：

1. 搜索现有 [Issues](https://github.com/Torther/rustrans/issues)
2. 创建新的 Issue
3. 联系维护者

---

⭐ 如果这个项目对您有帮助，请给我们一个星标！
