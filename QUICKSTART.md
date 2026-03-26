# 快速开始指南

本指南帮助您在本地快速启动和运行React + Rust企业级后台管理系统。

## 🚀 本地开发环境

### 环境要求

- **Rust**: 1.70+
- **Node.js**: 18+
- **PostgreSQL**: 14+
- **Docker**: 20.10+ (可选)

## 📦 安装依赖

### 1. 安装Rust

```bash
# Linux/macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows
# 下载并运行 rustup-init.exe

# 验证安装
rustc --version
cargo --version
```

### 2. 安装Node.js

```bash
# 使用nvm安装（推荐）
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 18
nvm use 18

# 验证安装
node --version
npm --version
```

### 3. 安装PostgreSQL

**macOS (使用Homebrew):**
```bash
brew install postgresql@15
brew services start postgresql@15
```

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install postgresql postgresql-contrib
sudo systemctl start postgresql
```

**Windows:**
下载并安装 PostgreSQL 15: https://www.postgresql.org/download/windows/

## 🗄️ 数据库设置

### 1. 创建数据库

```bash
# 连接到PostgreSQL
psql -U postgres

# 创建数据库
CREATE DATABASE kao_db;

# 退出
\q
```

### 2. 运行迁移脚本

```bash
cd backend

# 按顺序执行迁移脚本
psql -U postgres -d kao_db -f migrations/0001_create_sys_department.sql
psql -U postgres -d kao_db -f migrations/0002_create_sys_post.sql
psql -U postgres -d kao_db -f migrations/0003_create_sys_user.sql
psql -U postgres -d kao_db -f migrations/0004_create_sys_role.sql
psql -U postgres -d kao_db -f migrations/0005_create_sys_menu.sql
psql -U postgres -d kao_db -f migrations/0006_create_sys_user_role.sql
psql -U postgres -d kao_db -f migrations/0007_create_sys_role_menu.sql
psql -U postgres -d kao_db -f migrations/0008_create_sys_role_department.sql
psql -U postgres -d kao_db -f migrations/0099_init_data.sql
```

## ⚙️ 后端配置

### 1. 配置环境变量

```bash
cd backend

# 复制示例配置
cp .env.example .env

# 编辑配置
vim .env
```

配置内容:
```env
DATABASE_URL=postgres://postgres:password@localhost:5432/kao_db
JWT_SECRET=your-secret-key
JWT_ACCESS_TOKEN_EXPIRES_IN=3600
JWT_REFRESH_TOKEN_EXPIRES_IN=604800
APP_PORT=8080
RUST_LOG=info
```

### 2. 安装Rust依赖

```bash
cd backend
cargo build
```

### 3. 运行后端

```bash
# 开发模式（热重载）
cargo run

# 或者
cargo watch -x run
```

后端将在 http://localhost:8080 启动

## 🎨 前端配置

### 1. 安装前端依赖

```bash
cd frontend
npm install
```

### 2. 配置环境变量

```bash
# 复制示例配置
cp .env.example .env

# 编辑配置
vim .env
```

配置内容:
```env
VITE_API_BASE_URL=http://localhost:8080
```

### 3. 运行前端

```bash
cd frontend
npm run dev
```

前端将在 http://localhost:3000 启动

## 🐳 Docker部署（可选）

### 使用Docker Compose快速启动

```bash
# 复制环境变量
cp .env.example .env

# 启动所有服务
docker-compose up -d

# 查看服务状态
docker-compose ps
```

服务将在以下地址启动:
- 前端: http://localhost
- 后端: http://localhost:8080
- 数据库: localhost:5432

## 🔑 登录系统

启动所有服务后，访问 http://localhost:3000

使用默认账号登录:
- **用户名**: `admin`
- **密码**: `admin123`

## 📚 开发指南

### 后端开发

```bash
# 编译检查
cargo check

# 运行测试
cargo test

# 构建发布版本
cargo build --release

# 代码格式化
cargo fmt

# 代码检查
cargo clippy
```

### 前端开发

```bash
# 开发服务器
npm run dev

# 构建生产版本
npm run build

# 代码检查
npm run lint

# 代码格式化
npm run format

# 类型检查
npm run type-check

# 运行测试
npm test
```

## 🏗️ 项目结构

```
├── backend/                    # Rust后端
│   ├── src/
│   │   ├── api/              # API路由层
│   │   │   ├── auth/         # 认证API
│   │   │   └── system/        # 系统管理API
│   │   ├── middleware/        # 中间件
│   │   ├── config/           # 配置管理
│   │   └── utils/            # 工具函数
│   ├── migrations/           # 数据库迁移
│   ├── Cargo.toml
│   └── Dockerfile
│
├── frontend/                  # React前端
│   ├── src/
│   │   ├── pages/            # 页面组件
│   │   ├── services/         # API服务
│   │   ├── components/        # 组件
│   │   ├── lib/              # 工具库
│   │   └── routes/           # 路由
│   ├── package.json
│   └── Dockerfile
│
├── docker-compose.yml         # Docker编排
├── .env.example              # 环境变量示例
└── README.md
```

## 🎯 下一步

1. 探索系统功能
2. 创建新用户和角色
3. 配置权限
4. 自定义菜单
5. 开始业务开发

## ❓ 常见问题

### 数据库连接失败

检查 `DATABASE_URL` 是否正确:
```bash
# 测试连接
psql <DATABASE_URL>
```

### 前端无法连接后端

确保后端正在运行并检查 `VITE_API_BASE_URL` 配置。

### Rust编译错误

确保安装最新稳定版Rust:
```bash
rustup update stable
```

### Node依赖安装失败

清除缓存后重试:
```bash
rm -rf node_modules package-lock.json
npm install
```

## 📞 获取帮助

- 查看 README.md 文档
- 查看 DEPLOYMENT.md 部署指南
- 提交 GitHub Issue
