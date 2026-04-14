---
artifact: prd
task: code-review-fixes
date: 2026-04-03
role: tech-lead
status: draft
---

# PRD: 代码审查问题修复

## 1. 背景

代码审查发现 41 个文件中有多个质量问题需要修复，包括：
- 安全性风险（SQL 模式、错误消息泄露）
- 死代码（app.rs 未使用的函数）
- 前端 mock 数据问题
- 代码重复

## 2. 目标与成功标准

### 目标
修复所有 Critical 和 Warning 级别问题

### 成功标准
- [ ] SQL WHERE 动态构建改用白名单校验
- [ ] auth/service.rs session expire_time 正确更新
- [ ] app.rs 死代码移除或标记实现
- [ ] 前端 fetchMetrics 移除 mock 实现真实解析
- [ ] job/repo.rs 分页逻辑提取为公共函数
- [ ] cleanup_expired_sessions 接入调度或移除

## 3. 用户故事

无用户故事，纯技术债务。

## 4. 范围

### In Scope
- backend/src/features/job/repo.rs - SQL 安全修复
- backend/src/features/auth/service.rs - session expire_time 修复
- backend/src/app.rs - 死代码移除
- frontend/src/services/api/monitoring.ts - mock 移除

### Out of Scope
- 新功能开发
- UI 变更
- 架构调整

## 5. 风险与缓解

| 风险 | 影响 | 缓解措施 |
|------|------|----------|
| 动态 SQL 构建 | 安全风险 | 白名单校验 |
| session expire_time 不更新 | 用户被迫提前登出 | 添加 ON CONFLICT UPDATE expire_time |

## 6. 待确认项

- [ ] job routes 权限检查是否在此次范围？
- [ ] cleanup_expired_sessions 是否需要定时调度？
