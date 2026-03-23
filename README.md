# AI Coding Project

一个面向企业后台管理场景的全栈脚手架。前端采用 `Vite + React 19 + TanStack Router`，后端采用 `Rust + Axum + SQLx + PostgreSQL`，数据库运行时真源是 `backend/migrations/`。

## 技术栈

| 层级 | 技术 |
|---|---|
| 前端 | React 19, Vite, TypeScript, TanStack Router, TanStack Query, React Hook Form, Zod, Tailwind CSS |
| 后端 | Rust 1.85+, Axum 0.8, Tokio, SQLx 0.8, JWT, bcrypt, Validator |
| 数据库 | PostgreSQL 16, SQL migrations |
| 工具 | Claude Code, Cursor, GitHub Copilot |

## 快速开始

前置要求：
- Node.js 22+
- Rust 1.85+
- Docker & Docker Compose

启动本地开发环境：

```bash
cp .env.example .env
docker compose up -d postgres
cd backend && cargo run
cd frontend && npm install && npm run dev
```

默认端口：
- 前端 `http://127.0.0.1:5173`
- 后端 `http://127.0.0.1:3001`
- PostgreSQL `127.0.0.1:5432`

## 项目结构

```text
.
├── frontend/     # Vite + React 前端
├── backend/      # Rust + Axum 后端
├── database/     # Prisma 风格参考模型
├── scripts/      # 初始化与计划脚本
├── .agent/       # AI Agent 配置
└── tests/        # 测试文件
```

## 环境变量

复制 `.env.example` 为 `.env` 后修改即可。前端读取 `VITE_API_BASE_URL`，后端读取 `HOST`、`APP_ENV`、`PORT`、`DATABASE_URL` 和 `JWT_SECRET`。

## API 概览

当前框架已提供的核心 API：
- `POST /api/v1/auth/login`
- `GET /api/v1/auth/profile`
- `GET /api/v1/auth/permissions`
- `GET /api/v1/auth/menus`
- `GET /api/v1/health`

后续会继续扩展用户、角色、部门、菜单等管理接口。

## 数据约定

- `backend/migrations/` 是运行时唯一真源
- `database/schema.prisma` 只作为参考模型
- SQLx 负责运行时代码访问
- UUID 由应用层生成或在种子中显式提供

## 开发流程

1. 先看风险和影响面
2. 读代码和约束文件
3. 拆任务，分配给 subagent
4. 先测后改，保留可验证结果
5. 完成后再考虑部署

## 常用命令

```bash
cd backend && cargo test
cd backend && cargo check
cd frontend && npm test -- --run
cd frontend && npm run build
cd frontend && npm run lint
docker compose up -d postgres
```
