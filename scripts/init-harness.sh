#!/bin/bash

# AI Coding Project - 初始化脚本

set -euo pipefail

echo "🚀 初始化 AI Coding Project..."

# 1. 检查 Node.js 版本
node_version=$(node -v | cut -d'v' -f2 | cut -d'.' -f1)
if [ "$node_version" -lt 22 ]; then
  echo "❌ Node.js 版本需要 22+，当前版本: $(node -v)"
  exit 1
fi

# 2. 检查 Rust 版本
rust_version=$(rustc --version | awk '{print $2}')
rust_major=$(printf '%s' "$rust_version" | cut -d'.' -f1)
rust_minor=$(printf '%s' "$rust_version" | cut -d'.' -f2)
if [ "$rust_major" -lt 1 ] || { [ "$rust_major" -eq 1 ] && [ "$rust_minor" -lt 85 ]; }; then
  echo "❌ Rust 版本需要 1.85+，当前版本: $(rustc --version)"
  exit 1
fi

# 3. 安装前端依赖
echo "📦 安装前端依赖..."
cd frontend
npm install
cd ..

# 4. 复制环境变量
echo "📝 复制环境变量..."
if [ ! -f .env ]; then
  cp .env.example .env
fi

# 5. 启动数据库
echo "🗄️ 启动数据库..."
docker compose up -d postgres

# 6. 预热后端构建
echo "🔧 检查后端构建..."
cd backend
cargo build
cd ..

echo "🧪 后端可测试状态已准备好"

echo "✅ 初始化完成!"
echo ""
echo "下一步:"
echo "  - 编辑 .env 文件配置数据库连接和 JWT 密钥"
echo "  - 运行 backend: cargo run"
echo "  - 运行 frontend: npm run dev"
