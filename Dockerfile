# ============================================
# 构建阶段
# ============================================
ARG RUST_VERSION=1.91.0
ARG APP_NAME=Rustrans

FROM rust:${RUST_VERSION}-slim-bullseye AS build
ARG APP_NAME
WORKDIR /app

# 安装构建依赖
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# 使用 BuildKit 缓存挂载优化构建速度
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=static,target=static \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
cargo build --locked --release
cp ./target/release/$APP_NAME /Rustrans
EOF

# ============================================
# 运行阶段
# ============================================
FROM debian:bullseye-slim AS final

# 安装运行时依赖（SSL 库和 CA 证书）
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl1.1 && \
    rm -rf /var/lib/apt/lists/*

# 创建非特权用户
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser

# 设置工作目录
WORKDIR /app

# 从构建阶段复制二进制文件
COPY --from=build --chown=appuser:appuser /Rustrans /app/Rustrans

# 复制静态文件
COPY --chown=appuser:appuser static ./static

# 确保 /app 目录及其内容的所有权正确
RUN chown -R appuser:appuser /app

# 切换到非特权用户
USER appuser

# 暴露端口
EXPOSE 9999

# 启动应用
CMD ["/app/Rustrans"]
