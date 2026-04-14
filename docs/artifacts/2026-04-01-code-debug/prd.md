---
artifact: prd
task: code-debug
date: 2026-04-01
role: tech-lead
status: completed
---

# 需求简报

## 背景
用户要求调试当前所有的前后端代码，确保满足用户需求。

## 目标与优先级

| 优先级 | 目标 |
|--------|------|
| P0 | 确认当前代码可正常编译运行 |
| P1 | 识别并修复已确认的 bug |
| P2 | 优化警告信息（可选） |

## 约束条件
- 必须满足用户需求
- 不得随意修改代码
- 仅修复明确认定的 bug
- 不做功能增强或重构

## 当前代码状态

### 编译状态
- **Backend (Rust)**: BUILD SUCCESSFUL（22 warnings, 0 errors）
- **Frontend (TypeScript/React)**: BUILD SUCCESSFUL（0 errors）

### 测试状态
- E2E 测试：PASSED
- 无失败的测试用例

## 已确认并修复的 Bug

### Bug 1 (CRITICAL): Token 字段名不匹配
**问题**: 后端登录返回 `token`，前端期望 `access_token`，导致登录后所有认证请求失败。

| 文件 | 问题 |
|------|------|
| `backend/src/features/auth/routes.rs:131` | 返回 `"token": "..."` |
| `frontend/src/lib/api.ts:35` | 读取 `localStorage.getItem('access_token')` |
| `frontend/src/types/auth.ts:24` | `LoginResponse` 期望 `access_token` |

**修复**: 修改后端 `routes.rs` 返回符合前端期望的字段格式：
```rust
"access_token": response.token,
"refresh_token": "",
"token_type": "Bearer",
"expires_in": 86400,
"user": response.userInfo
```

**状态**: ✅ 已修复

### Bug 2 (HIGH): Monitoring API 使用原生 axios 无认证
**问题**: `monitoring.ts` 使用原生 `axios` 而非带 auth interceptor 的实例，导致 monitoring 相关 API 请求没有携带 token。

| 文件 | 问题 |
|------|------|
| `frontend/src/services/api/monitoring.ts:1` | `import axios from 'axios'` |

**修复**: 创建带 auth interceptor 的 `authApi` 实例，所有需要认证的 monitoring API 改用 `authApi`：
- `fetchOperationLogs` / `createOperationLog` / `getOperationLog` / `deleteOperationLog`
- `fetchLoginLogs` / `getLoginLog`
- `fetchOnlineUsers` / `forceLogout`

**状态**: ✅ 已修复

### Bug 3 (HIGH): Security API 使用原生 axios 无认证
**问题**: `security.ts` 使用原生 `axios` 而非带 auth interceptor 的实例，导致安全监控 API 请求没有携带 token。

| 文件 | 问题 |
|------|------|
| `frontend/src/services/api/security.ts:1` | `import axios from 'axios'` |

**修复**: 创建带 auth interceptor 的 `authApi` 实例，所有安全监控 API 改用 `authApi`：
- `fetchSecurityScan()`
- `fetchSecurityScanByType()`
- `fetchSecurityEvents()`
- `fetchPasswordHealth()`

**状态**: ✅ 已修复

### 已识别但未修复（非 bug）
以下属于代码风格警告，**不是 bug**：
- Rust `unused imports/variables` - 部分模块存在未使用的代码
- Rust `non_snake_case` - userInfo 字段命名
- 前端 chunk size 警告（antd 库大小 1MB+）

## 参与角色

| 角色 | 职责 |
|------|------|
| tech-lead | 统筹协调，bug 确认 |
| backend-engineer | 分析后端 Rust 代码 |
| frontend-engineer | 分析前端 React/TypeScript 代码 |

## 领域技能包

本次任务未涉及以下领域：
- BPMN 流程引擎
- HPRMC 权限中心
- 海尔企业 SDK
- GitLab 发布
- Langfuse 追踪
- DDD 业务服务设计

## UI 质量门禁

已完成所有 16 个页面的浏览器 smoke 测试：

### 测试通过的页面
| 页面 | 路径 | 截图 |
|------|------|------|
| Dashboard | /dashboard | `smoke-dashboard.png` |
| 用户管理 | /system/users | `smoke-user-management.png` |
| 部门管理 | /system/departments | `smoke-department-management.png` |
| 角色管理 | /system/roles | `smoke-role-management.png` |
| 菜单管理 | /system/menus | `smoke-menu-management.png` |
| 岗位管理 | /system/posts | `smoke-post-management.png` |
| 字典类型 | /dictionary/type | `smoke-dictionary-type.png` |
| 字典数据 | /dictionary/data | `smoke-dictionary-data.png` |
| 参数配置 | /config | `smoke-config.png` |
| 通知公告 | /notice | `smoke-notice.png` |
| 定时任务 | /job | `smoke-job-scheduler.png` |
| 任务日志 | /job/log | `smoke-job-log.png` |
| 安全监控 | /monitoring/security | `smoke-security-monitoring-fixed.png` ✅ |
| 在线用户 | /monitoring/online-user | `smoke-online-users.png` |
| 操作日志 | /monitoring/operation-log | `smoke-operation-log.png` |
| 登录日志 | /monitoring/login-log | `smoke-login-log.png` |

✅ 安全监控页面已修复 - 使用正确的 auth axios 实例
`docs/artifacts/2026-04-01-code-debug/screenshots/`

## 下一步行动

1. ✅ **Bug 修复已完成**: Token 字段名不匹配、Monitoring API 认证问题、Security API 认证问题已修复
2. ✅ **Smoke 测试完成**: 所有 16 个页面正常显示
3. **验证**: 请重新登录测试，确认各功能页面可正常请求后端 API
4. **如有其他问题**: 请提供具体的错误信息或不符合预期的行为
