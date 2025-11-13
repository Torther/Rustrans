# 安全性配置和最佳实践

## 依赖安全审计

为了确保项目的安全性，请定期运行以下命令：

```bash
# 安装安全审计工具
cargo install cargo-audit
cargo install cargo-deny

# 检查已知漏洞
cargo audit

# 检查许可证合规性和安全策略
cargo deny check
```

## 配置文件安全

### 1. 文件权限
确保配置文件具有适当的权限：
```bash
chmod 600 config.json
```

### 2. 环境变量
对于生产环境，建议使用环境变量存储敏感信息：
```bash
export RUSTRANS_API_KEY="your-secure-api-key"
export RUSTRANS_API_URL="https://api.example.com/v1/chat/completions"
export RUSTRANS_MODEL="gpt-4"
```

### 3. Docker 安全
- 使用非 root 用户运行容器
- 最小化镜像大小
- 定期更新基础镜像

## API 安全

### 1. 输入验证
- 文本长度限制：1-10,000 字符
- 名称长度限制：1-100 字符
- 目标语言数量限制：1-10 个

### 2. 速率限制
建议在生产环境中添加速率限制中间件。

### 3. CORS 配置
当前配置允许所有来源，生产环境中应该限制为特定域名。

## 运行时安全

### 1. 内存安全
Rust 的内存安全特性防止了常见的内存漏洞。

### 2. 错误处理
使用结构化的错误处理，避免信息泄露。

### 3. 日志安全
- 避免在日志中记录敏感信息
- 使用适当的日志级别

## 建议的安全改进

1. **添加 API 密钥加密**
2. **实现请求签名验证**
3. **添加请求速率限制**
4. **实施 IP 白名单/黑名单**
5. **添加 CSRF 保护**
6. **使用 HTTPS 强制加密**