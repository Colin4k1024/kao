# React + Rust 企业级后台管理系统

基于 React + Rust 的类若依(RuoYi)企业级后台管理系统。

## 📋 项目概述

这是一个高度可伸缩、安全内生且可长期维护的后台管理系统，采用现代化的全栈技术栈。

### 技术栈

**后端技术栈**
- Rust + Axum Web框架
- Tokio 异步运行时
- SQLx 数据库ORM
- PostgreSQL 数据库
- JWT 认证
- bcrypt 密码加密

**前端技术栈**
- React 19 + Vite
- TypeScript 严格模式
- Ant Design UI
- React Router 路由
- Axios HTTP客户端
- TanStack Query 状态管理

## 🎯 功能模块

### ✅ 已完成功能

#### 阶段一：项目基础架构搭建
- [x] Rust后端项目初始化
- [x] React前端项目初始化
- [x] 项目目录结构规范化

#### 阶段二：数据库设计与迁移
- [x] RBAC权限相关表（8个表）
- [x] 动态配置表（4个表）
- [x] 系统运维表（5个表）
- [x] 数据库迁移脚本
- [x] 初始化数据

#### 阶段三：后端框架核心
- [x] 配置层实现
- [x] 认证模块实现（登录/注册/登出/Token刷新）
- [x] 公共响应结构
- [x] 中间件实现（认证/日志/错误处理/限流）

#### 阶段四：RBAC权限核心模块
- [x] 用户管理CRUD API
- [x] 部门管理CRUD API（树形结构）
- [x] 岗位管理CRUD API
- [x] 角色管理CRUD API
- [x] 菜单管理CRUD API（树形结构）

#### 阶段五：前端核心框架
- [x] API请求封装（Axios + 拦截器）
- [x] 认证服务
- [x] 系统服务（用户/部门/岗位/角色/菜单）
- [x] 路由配置
- [x] 主布局组件
- [x] 路由守卫

#### 阶段六：系统管理前端模块
- [x] 用户管理页面（列表/新增/编辑/删除/重置密码/角色分配）
- [x] 部门管理页面（树形结构展示）
- [x] 岗位管理页面
- [x] 角色管理页面（菜单权限分配）
- [x] 菜单管理页面

### ⏳ 待完成功能

#### 阶段七：动态配置模块
- [ ] 字典管理（类型/数据）
- [ ] 参数管理
- [ ] 通知公告管理

#### 阶段八：定时任务系统
- [ ] 任务调度后端
- [ ] 任务管理前端

#### 阶段九：系统监控
- [ ] 系统指标监控
- [ ] 操作日志
- [ ] 在线用户管理

#### 阶段十：测试与部署
- [ ] 单元测试
- [ ] 集成测试
- [ ] Docker部署配置
- [ ] 性能优化

## 🚀 快速开始

### 环境要求
- Rust 1.70+
- Node.js 18+
- PostgreSQL 14+

### 后端启动

```bash
cd backend

# 复制环境配置
cp .env.example .env
# 编辑 .env 配置数据库连接

# 运行数据库迁移
psql -U postgres -d kao_db -f migrations/0001_create_sys_department.sql
psql -U postgres -d kao_db -f migrations/0002_create_sys_post.sql
psql -U postgres -d kao_db -f migrations/0003_create_sys_user.sql
psql -U postgres -d kao_db -f migrations/0004_create_sys_role.sql
psql -U postgres -d kao_db -f migrations/0005_create_sys_menu.sql
psql -U postgres -d kao_db -f migrations/0006_create_sys_user_role.sql
psql -U postgres -d kao_db -f migrations/0007_create_sys_role_menu.sql
psql -U postgres -d kao_db -f migrations/0008_create_sys_role_department.sql
psql -U postgres -d kao_db -f migrations/0099_init_data.sql

# 启动后端服务
cargo run
```

### 前端启动

```bash
cd frontend

# 安装依赖
npm install

# 启动开发服务器
npm run dev
```

### 访问系统

- 前端地址: http://localhost:3000
- 后端地址: http://localhost:8080
- 默认账号: `admin` / `admin123`

## 📁 项目结构

```
├── backend/                    # Rust后端
│   ├── src/
│   │   ├── api/              # API路由层
│   │   │   ├── auth/         # 认证相关API
│   │   │   └── system/        # 系统管理API
│   │   ├── middleware/        # 中间件
│   │   ├── config/           # 配置管理
│   │   └── utils/            # 工具函数
│   ├── migrations/           # 数据库迁移脚本
│   └── Cargo.toml
│
├── frontend/                  # React前端
│   ├── src/
│   │   ├── pages/            # 页面组件
│   │   │   ├── Login.tsx
│   │   │   ├── Dashboard.tsx
│   │   │   └── system/       # 系统管理页面
│   │   ├── services/         # API服务层
│   │   ├── components/        # 组件
│   │   ├── lib/              # 工具库
│   │   └── routes/           # 路由配置
│   └── package.json
│
└── README.md
```

## 🔑 核心功能

### 用户管理
- 用户CRUD操作
- 用户状态管理（启用/禁用）
- 密码修改/重置
- 角色分配
- 部门分配

### 权限管理
- RBAC权限模型
- 菜单权限分配
- 数据范围控制
- 按钮级别权限

### 部门管理
- 树形结构展示
- 层级关系维护
- 部门负责人设置

## 🔒 安全性

- JWT Token认证
- bcrypt密码加密
- SQL参数化查询防注入
- 请求限流保护

## 📈 性能优化

- 数据库连接池
- 前端路由懒加载
- API请求缓存
- 分页查询优化

## 📝 API文档

### 认证接口

| 方法 | 路径 | 描述 |
|------|------|------|
| POST | /api/auth/login | 用户登录 |
| POST | /api/auth/register | 用户注册 |
| POST | /api/auth/logout | 退出登录 |
| POST | /api/auth/refresh | 刷新Token |

### 用户管理

| 方法 | 路径 | 描述 |
|------|------|------|
| GET | /api/system/users | 获取用户列表 |
| GET | /api/system/users/:id | 获取用户详情 |
| POST | /api/system/users | 创建用户 |
| PUT | /api/system/users/:id | 更新用户 |
| DELETE | /api/system/users/:id | 删除用户 |
| PUT | /api/system/users/:id/reset-password | 重置密码 |
| PUT | /api/system/users/:id/roles | 分配角色 |

### 其他模块

类似的RESTful API设计模式应用于：
- 部门管理 (/api/system/departments)
- 岗位管理 (/api/system/posts)
- 角色管理 (/api/system/roles)
- 菜单管理 (/api/system/menus)

## 📊 数据库表

### RBAC权限表
- `sys_user` - 用户表
- `sys_department` - 部门表
- `sys_post` - 岗位表
- `sys_role` - 角色表
- `sys_menu` - 菜单表
- `sys_user_role` - 用户角色关联表
- `sys_role_menu` - 角色菜单关联表
- `sys_role_department` - 角色部门关联表

### 动态配置表
- `sys_dict_type` - 字典类型表
- `sys_dict_data` - 字典数据表
- `sys_config` - 参数配置表
- `sys_notice` - 通知公告表

### 系统运维表
- `sys_job` - 定时任务表
- `sys_job_log` - 任务日志表
- `sys_oper_log` - 操作日志表
- `sys_login_log` - 登录日志表
- `sys_online_user` - 在线用户表

## 🤝 贡献指南

欢迎提交Issue和Pull Request！

## 📄 许可证

MIT License

## 📞 联系方式

如有问题，请提交Issue。
