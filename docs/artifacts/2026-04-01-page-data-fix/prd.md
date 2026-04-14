---
artifact: prd
task: page-data-fix
date: 2026-04-01
role: tech-lead
status: draft
---

# 页面数据缺失问题 - 解决计划

## 问题概述

数据库表中有数据，但前端页面显示"暂无数据"。

### 已验证的 API 响应

| 页面 | API 路径 | 后端返回 | 问题 |
|------|---------|---------|------|
| 用户管理 | `/api/v1/users` | `{ items: [], total: N }` | ✅ 正常 |
| 部门管理 | `/api/v1/departments` | `[]` (数组) | ❌ 前端期望 `data.list` |
| 角色管理 | `/api/v1/roles` | `[]` | ❌ |
| 菜单管理 | `/api/v1/menus` | `[]` | ❌ |
| 岗位管理 | `/api/v1/posts` | `[]` | ❌ |
| 字典类型 | `/api/system/dict/type` | `[]` | ❌ |
| 字典数据 | `/api/system/dict/data` | `[]` | ❌ |
| 参数配置 | `/api/system/config` | `[]` | ❌ |
| 通知公告 | `/api/system/notice` | `[]` | ❌ |
| 定时任务 | `/api/v1/jobs` | 待验证 | - |

### 根本原因

**后端返回格式不一致**：

前端 `request.get<T>()` 始终返回 `res.data.data`，当后端返回数组时：
- 后端响应：`{ code: 200, message: "ok", data: [...] }`
- `request.get()` 提取后：`res.data.data` = 数组
- 前端代码期望：`data.list` 或 `data.items`
- 结果：`undefined`，显示"暂无数据"

## 解决方案

### 方案：修改后端返回格式统一为 `{ items: [], total: N }`

### 修复计划

| # | 页面 | 后端文件 | 当前返回 | 修改为 |
|---|------|---------|---------|--------|
| 1 | 部门管理 | `backend/src/features/departments/routes.rs:48` | `ApiResponse::success(departments)` | `ApiResponse::success(json!({"items": departments, "total": departments.len()}))` |
| 2 | 角色管理 | `backend/src/features/roles/routes.rs` | `ApiResponse::success(roles)` | 同上格式 |
| 3 | 菜单管理 | `backend/src/features/menus/routes.rs` | `ApiResponse::success(menus)` | 同上格式 |
| 4 | 岗位管理 | `backend/src/features/posts/routes.rs` | `ApiResponse::success(posts)` | 同上格式 |
| 5 | 字典类型 | `backend/src/features/dictionary/type/routes.rs` | `ApiResponse::success(types)` | 同上格式 |
| 6 | 字典数据 | `backend/src/features/dictionary/data/routes.rs` | `ApiResponse::success(data)` | 同上格式 |
| 7 | 参数配置 | `backend/src/features/config/routes.rs` | `ApiResponse::success(configs)` | 同上格式 |
| 8 | 通知公告 | `backend/src/features/notice/routes.rs` | `ApiResponse::success(notices)` | 同上格式 |

## 风险与约束

- 仅修改后端返回格式，保持前端不变
- 不做功能增强或重构
- 修复后需验证所有页面能正确显示数据
