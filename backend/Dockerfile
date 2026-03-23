FROM rust:1.85-alpine AS builder

WORKDIR /app

# 安装编译依赖
RUN apk add --no-cache musl-dev openssl-dev pkgconfig

# 复制依赖文件
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# 构建
RUN cargo build --release

# 运行镜像
FROM alpine:3.19

WORKDIR /app

# 安装运行时依赖
RUN apk add --no-cache ca-certificates libssl3

# 复制编译产物
COPY --from=builder /app/target/release/ai-coding-backend /app/

# 暴露端口
EXPOSE 3001

# 运行
CMD ["/app/ai-coding-backend"]
