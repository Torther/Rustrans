# Rustrans Makefile

.PHONY: help build test lint check format clean docker-build docker-run security ci

# 默认目标
help:
	@echo "Rustrans 翻译服务"
	@echo ""
	@echo "可用命令:"
	@echo "  build        - 构建项目"
	@echo "  test         - 运行测试"
	@echo "  test-coverage- 运行测试并生成覆盖率报告"
	@echo "  lint         - 运行代码检查"
	@echo "  check        - 快速检查代码"
	@echo "  format       - 格式化代码"
	@echo "  clean        - 清理构建文件"
	@echo "  docker-build - 构建 Docker 镜像"
	@echo "  docker-run   - 运行 Docker 容器"
	@echo "  security     - 运行安全审计"
	@echo "  ci           - 运行 CI 流水线"

# 构建项目
build:
	@echo "🔨 构建项目..."
	cargo build --release

# 运行测试
test:
	@echo "🧪 运行测试..."
	cargo test

# 运行测试并生成覆盖率报告
test-coverage:
	@echo "📊 运行测试并生成覆盖率报告..."
	cargo install cargo-tarpaulin --quiet || true
	cargo tarpaulin --out html --output-dir target/tarpaulin

# 代码检查
lint:
	@echo "🔍 运行 Clippy..."
	cargo clippy --all-targets --all-features -- -D warnings

# 快速检查
check:
	@echo "⚡ 快速检查..."
	cargo check

# 格式化代码
format:
	@echo "📝 格式化代码..."
	cargo fmt

# 清理构建文件
clean:
	@echo "🧹 清理构建文件..."
	cargo clean
	rm -rf target/tarpaulin

# Docker 构建
docker-build:
	@echo "🐳 构建 Docker 镜像..."
	docker build -t rustrans:latest .

# Docker 运行
docker-run:
	@echo "🚀 运行 Docker 容器..."
	docker-compose up -d

# 停止 Docker 容器
docker-stop:
	@echo "⏹️ 停止 Docker 容器..."
	docker-compose down

# 安全审计
security:
	@echo "🔒 运行安全审计..."
	@if ! command -v cargo-audit &> /dev/null; then \
		echo "安装 cargo-audit..."; \
		cargo install cargo-audit --quiet; \
	fi
	cargo audit
	@if ! command -v cargo-deny &> /dev/null; then \
		echo "安装 cargo-deny..."; \
		cargo install cargo-deny --quiet; \
	fi
	cargo deny check

# 安装开发工具
install-tools:
	@echo "🛠️ 安装开发工具..."
	cargo install cargo-audit cargo-deny cargo-outdated cargo-watch --quiet

# 开发模式运行
dev:
	@echo "🔄 开发模式运行..."
	RUST_LOG=debug cargo watch -x run

# 更新依赖
update-deps:
	@echo "⬆️ 更新依赖..."
	cargo update

# 检查过期依赖
check-outdated:
	@echo "📅 检查过期依赖..."
	@if ! command -v cargo-outdated &> /dev/null; then \
		echo "安装 cargo-outdated..."; \
		cargo install cargo-outdated --quiet; \
	fi
	cargo outdated

# 生产构建（优化）
build-prod:
	@echo "🏗️ 生产构建..."
	cargo build --release --target x86_64-unknown-linux-musl

# CI 流水线
ci: check lint test security
	@echo "✅ CI 流水线完成"

# 生成 API 文档
docs:
	@echo "📚 生成 API 文档..."
	cargo doc --no-deps --open

# 性能测试
bench:
	@echo "⚡ 运行性能测试..."
	cargo bench

# 安装预提交钩子
install-hooks:
	@echo "🪝 安装预提交钩子..."
	@if [ -d ".git" ]; then \
		echo '#!/bin/sh' > .git/hooks/pre-commit; \
		echo 'make check lint test' >> .git/hooks/pre-commit; \
		chmod +x .git/hooks/pre-commit; \
		echo "预提交钩子已安装"; \
	fi

# 创建发布包
package:
	@echo "📦 创建发布包..."
	mkdir -p dist
	cp target/release/Rustrans dist/
	cp config.json.example dist/config.json.example 2>/dev/null || true
	cp README.md LICENSE dist/
	cp docker-compose.yml dist/