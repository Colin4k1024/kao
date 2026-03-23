# Backend 开发规范 (Rust + Axum)

## 技术栈
- 运行时：Rust
- Web 框架：Axum
- 异步运行时：Tokio
- 认证：JWT (jsonwebtoken)
- 数据库：SQLx + PostgreSQL
- 验证：Validator

## API 设计
- 遵循 RESTful 规范
- 版本化 API：/api/v1/*
- 错误码统一：200 成功，400 参数错误，401 未认证，403 无权限，500 服务错误

## 项目结构
```
src/
├── main.rs           # 入口
├── app/              # 应用组装与状态
├── common/           # 配置、错误、JWT、权限
└── features/         # auth / users / roles / departments / menus
```

## 认证
- JWT Token
- Token 放入 Authorization Header
-  Bearer 模式

## 数据与模块
- 运行时代码使用 SQLx
- SQL 通过参数化查询执行
- `backend/migrations/` 是运行时真源
- 每个 feature 以 `model.rs`、`repo.rs`、`service.rs`、`routes.rs` 组织

## 错误处理
- 使用 Result<T, AppError>
- 统一错误响应格式
- 打印结构化日志
