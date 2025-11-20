#!/bin/bash

# CI 测试脚本 - 本地运行以验证 CI 步骤

set -e

echo "🚀 开始 CI 测试流程..."

# 检查格式
echo "📝 检查代码格式..."
cargo fmt --all -- --check

# 运行 clippy
echo "🔍 运行 Clippy 检查..."
cargo clippy --all-targets --all-features -- -D warnings

# 运行测试
echo "🧪 运行测试..."
cargo test --verbose

# 构建项目
echo "🔨 构建项目..."
cargo build --release

echo "✅ 所有检查通过！"
echo ""
echo "📊 构建产物:"
ls -la target/release/Rustrans

echo ""
echo "🐳 Docker 构建（可选）:"
echo "docker build -t rustrans:test ."
echo ""