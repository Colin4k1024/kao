# React + Rust 企业级后台管理系统规格说明书

## 概述

本系统是一个基于 React + Rust 技术栈的类若依(RuoYi)企业级后台管理系统，提供开箱即用的"数字底座"，将业务线开发人员从底层架构搭建与权限逻辑编写中解放出来。

## 技术栈

### 后端技术栈
- **语言**: Rust
- **Web框架**: Axum
- **异步运行时**: Tokio
- **数据库**: PostgreSQL/MySQL/SQLite
- **ORM**: SQLx (编译期安全的SQL校验) 或 SeaORM
- **日志**: tracing + tracing-subscriber
- **系统监控**: sysinfo

### 前端技术栈
- **框架**: React 19
- **构建工具**: Vite
- **语言**: TypeScript (严格模式)
- **UI库**: Tailwind CSS + Shadcn UI
- **状态管理**: TanStack Query (React Query)
- **表单管理**: React Hook Form + Zod
- **路由**: TanStack Router
- **HTTP客户端**: Axios/Fetch

## 核心模块

### 1. RBAC权限管理系统

#### 1.1 用户管理 (User Management)
- **功能**:
  - 用户CRUD操作
  - 用户状态管理（启用/禁用/锁定）
  - 密码管理（加密存储、重置）
  - 角色分配
  - 岗位分配
  - 部门分配
  - 最后登录信息记录
  - 在线用户管理

- **字段**:
  - 用户ID (UUID)
  - 用户名 (唯一)
  - 密码 (bcrypt加密)
  - 昵称
  - 邮箱
  - 手机号
  - 头像
  - 状态 (启用/禁用/锁定)
  - 部门ID (外键)
  - 岗位ID (外键)
  - 创建时间
  - 更新时间
  - 最后登录时间
  - 登录IP

#### 1.2 部门管理 (Department Management)
- **功能**:
  - 部门CRUD操作
  - 树形结构展示
  - 部门负责人设置
  - 部门状态管理
  - 部门排序
  - 祖籍链存储（用于高效查询子部门）

- **字段**:
  - 部门ID
  - 父部门ID (自引用外键)
  - 部门名称
  - 显示顺序
  - 负责人
  - 联系电话
  - 邮箱
  - 状态
  - 祖籍链 (ancestors)
  - 创建时间

#### 1.3 岗位管理 (Post Management)
- **功能**:
  - 岗位CRUD操作
  - 岗位状态管理
  - 岗位编码管理

- **字段**:
  - 岗位ID
  - 岗位编码
  - 岗位名称
  - 排序
  - 状态
  - 创建时间

#### 1.4 角色管理 (Role Management)
- **功能**:
  - 角色CRUD操作
  - 菜单权限分配
  - 数据范围配置
  - 角色状态管理
  - 角色标识（系统/自定义）

- **字段**:
  - 角色ID
  - 角色名称
  - 角色编码
  - 显示顺序
  - 状态
  - 角色类型 (系统角色/自定义角色)
  - 数据范围 (全部/本部门/本部门及以下/自定义)
  - 备注
  - 创建时间

#### 1.5 菜单管理 (Menu Management)
- **功能**:
  - 菜单CRUD操作
  - 菜单图标配置
  - 路由路径配置
  - 组件路径配置
  - 菜单类型（M目录/C菜单/F按钮）
  - 权限标识配置
  - 显示排序
  - 缓存配置
  - 是否可见

- **字段**:
  - 菜单ID
  - 父菜单ID
  - 菜单名称
  - 菜单类型
  - 图标
  - 路由路径
  - 组件路径
  - 权限标识
  - 显示排序
  - 是否缓存
  - 是否可见
  - 状态
  - 祖籍链

### 2. 动态配置系统

#### 2.1 字典管理 (Dictionary Management)
- **功能**:
  - 字典类型管理
  - 字典数据管理
  - 缓存支持
  - 导出导入

- **字段** (字典类型):
  - 类型ID
  - 类型名称
  - 类型编码
  - 状态

- **字段** (字典数据):
  - 数据ID
  - 类型编码
  - 字典标签
  - 字典键值
  - 排序
  - 样式
  - 状态
  - 备注

#### 2.2 参数管理 (Parameter Configuration)
- **功能**:
  - 参数CRUD
  - 参数分组
  - 参数类型支持
  - 敏感参数加密

- **字段**:
  - 参数ID
  - 参数名称
  - 参数键名
  - 参数值
  - 参数类型
  - 是否系统参数
  - 状态
  - 备注

#### 2.3 通知公告 (Notice Broadcasting)
- **功能**:
  - 公告CRUD
  - 富文本编辑
  - 发布/撤回
  - 置顶
  - 公告类型（通知/公告）

- **字段**:
  - 公告ID
  - 标题
  - 内容
  - 类型
  - 发布人
  - 发布时间
  - 公告状态
  - 是否置顶
  - 访问量

### 3. 定时任务系统 (Background Jobs)

- **功能**:
  - 任务CRUD
  - 任务调度表达式配置
  - 任务执行日志
  - 任务状态管理
  - 任务立即执行
  - 任务暂停/恢复

- **字段**:
  - 任务ID
  - 任务名称
  - 任务组
  - 调用目标
  - 执行表达式
  - 并发执行配置
  - 任务状态
  - 备注
  - 创建时间

### 4. 系统监控与日志

#### 4.1 系统监控
- **功能**:
  - 服务器指标采集 (CPU/内存/磁盘/网络)
  - 应用健康检查
  - 在线用户监控
  - 操作日志记录

- **端点**:
  - GET /api/system/metrics - 系统指标
  - GET /api/system/health - 健康检查
  - GET /api/system/online-users - 在线用户

#### 4.2 操作日志
- **功能**:
  - 操作日志记录
  - 日志查询
  - 日志导出

- **字段**:
  - 日志ID
  - 用户ID
  - 用户名
  - 操作类型
  - 请求方法
  - 请求URL
  - 请求参数
  - IP地址
  - 操作时间
  - 执行状态
  - 错误消息

## API设计规范

### 认证
- JWT Token认证
- Token刷新机制
- 单点登录支持

### 请求格式
```
Content-Type: application/json
Authorization: Bearer <token>
```

### 响应格式
```json
{
  "code": 200,
  "message": "success",
  "data": {}
}
```

### 分页响应
```json
{
  "code": 200,
  "message": "success",
  "data": {
    "list": [],
    "total": 100,
    "page": 1,
    "pageSize": 10
  }
}
```

## 数据库设计规范

### 表命名
- 统一使用snake_case
- 带前缀区分：sys_ (系统表)

### 主键策略
- 使用UUID作为主键

### 审计字段
- 所有表包含：
  - created_at
  - updated_at
  - created_by
  - updated_by

### 软删除
- 使用deleted_at字段实现软删除

## 前端项目结构

```
frontend/
├── src/
│   ├── components/
│   │   ├── ui/              # Shadcn UI组件
│   │   └── common/          # 通用组件
│   ├── features/
│   │   ├── system/          # 系统管理模块
│   │   │   ├── users/       # 用户管理
│   │   │   ├── departments/  # 部门管理
│   │   │   ├── roles/       # 角色管理
│   │   │   ├── menus/       # 菜单管理
│   │   │   └── posts/       # 岗位管理
│   │   ├── dictionary/      # 字典管理
│   │   ├── parameters/      # 参数管理
│   │   ├── notices/         # 通知公告
│   │   └── jobs/            # 定时任务
│   ├── hooks/               # 自定义Hooks
│   ├── lib/                 # 工具库
│   ├── routes/              # 路由配置
│   ├── services/            # API服务层
│   ├── stores/              # 状态管理
│   └── types/               # TypeScript类型定义
```

## 后端项目结构

```
backend/
├── src/
│   ├── api/
│   │   ├── system/          # 系统管理API
│   │   ├── auth/            # 认证API
│   │   └── ...
│   ├── models/              # 数据模型
│   ├── services/           # 业务逻辑层
│   ├── repositories/       # 数据访问层
│   ├── middleware/         # 中间件
│   ├── config/             # 配置管理
│   └── utils/              # 工具函数
├── migrations/            # 数据库迁移
└── Cargo.toml
```

## 安全性要求

### 认证与授权
- JWT Token有效期：Access Token (1小时) / Refresh Token (7天)
- 密码必须bcrypt加密存储
- 敏感操作需二次验证

### 输入验证
- 所有输入必须经过Zod/Zod验证
- SQL参数化查询防注入
- XSS防护

### 审计
- 记录所有关键操作日志
- 日志保留至少6个月

## 性能要求

- API响应时间 < 200ms (P95)
- 支持1000+并发用户
- 数据库连接池配置

## 部署要求

### 环境要求
- Rust 1.70+
- Node.js 18+
- PostgreSQL 14+ / MySQL 8+

### 容器化
- 提供Dockerfile和docker-compose.yml
- 环境变量配置敏感信息
