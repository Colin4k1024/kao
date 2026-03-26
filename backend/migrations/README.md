# 数据库迁移说明

## 迁移执行顺序

迁移脚本按编号顺序执行，从 `0001` 到 `0099`。以下是完整的迁移顺序：

### 第一阶段：基础表结构 (0001-0012)

| 序号 | 文件名 | 说明 |
|------|--------|------|
| 1 | `0001_init_rbac.sql` | RBAC基础架构：部门、用户、角色、菜单表 |
| 2 | `0002_seed_admin.sql` | 种子数据：初始管理员用户和基础菜单 |
| 3 | `0003_create_sys_user.sql` | 用户表（独立版本） |
| 4 | `0004_create_sys_role.sql` | 角色表（独立版本） |
| 5 | `0005_create_sys_menu.sql` | 菜单表（独立版本） |
| 6 | `0006_create_sys_user_role.sql` | 用户角色关联表 |
| 7 | `0007_create_sys_role_menu.sql` | 角色菜单关联表 |
| 8 | `0008_create_sys_role_department.sql` | 角色部门关联表 |
| 9 | `0009_create_sys_dict_type.sql` | 字典类型表 |
| 10 | `0010_create_sys_dict_data.sql` | 字典数据表 |
| 11 | `0011_create_sys_config.sql` | 系统配置表 |
| 12 | `0012_create_sys_notice.sql` | 系统公告表 |

### 第二阶段：运维日志表 (0013-0017)

| 序号 | 文件名 | 说明 |
|------|--------|------|
| 13 | `0013_create_sys_job.sql` | 定时任务表 |
| 14 | `0014_create_sys_job_log.sql` | 定时任务执行日志表 |
| 15 | `0015_create_sys_oper_log.sql` | 操作日志表 |
| 16 | `0016_create_sys_login_log.sql` | 登录日志表 |
| 17 | `0017_create_sys_online_user.sql` | 在线用户表 |

### 第三阶段：初始化数据 (0099)

| 序号 | 文件名 | 说明 |
|------|--------|------|
| 99 | `0099_init_data.sql` | 系统初始化数据 |

## 如何运行迁移

### 方式一：使用 psql 命令行

```bash
# 连接到数据库
psql -h <hostname> -U <username> -d <database>

# 执行单个迁移文件
\i 0001_init_rbac.sql
\i 0002_seed_admin.sql
# ... 以此类推

# 或按顺序执行所有迁移
\i 0003_create_sys_user.sql
\i 0004_create_sys_role.sql
# ... 依次执行
```

### 方式二：使用 SQL 文件批量执行

```bash
# 在项目根目录执行
cd backend/migrations

# 使用 find + psql 批量执行
find . -name "*.sql" -type f | sort | xargs -I {} psql -h <hostname> -U <username> -d <database> -f {}
```

### 方式三：使用 SQL Shell

```sql
-- 在数据库命令行中依次执行
\i 0001_init_rbac.sql
\i 0002_seed_admin.sql
-- 继续执行剩余文件
```

## 初始账号密码

### 超级管理员账户

| 属性 | 值 |
|------|-----|
| 用户名 | `admin` |
| 密码 | `admin123` |
| 邮箱 | `admin@example.com` |
| 联系电话 | `13800138000` |
| 所属部门 | 某某科技有限公司 |
| 岗位 | 首席执行官 |

### 密码说明

- 密码使用 bcrypt 加密
- 加密后的密码：`$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5CsIIc.x5T0S2`
- bcrypt 加密因子：12 轮

## 预定义数据

### 部门数据

| ID | 部门名称 | 父部门 | 负责人 |
|----|---------|--------|--------|
| `00000000-0000-0000-0000-000000000001` | 某某科技有限公司 | - | admin |
| `00000000-0000-0000-0000-000000000002` | 研发部 | 某某科技有限公司 | dev_leader |
| `00000000-0000-0000-0000-000000000003` | 销售部 | 某某科技有限公司 | sales_leader |

### 岗位数据

| ID | 岗位编码 | 岗位名称 |
|----|---------|---------|
| `00000000-0000-0000-0000-000000000010` | ceo | 首席执行官 |
| `00000000-0000-0000-0000-000000000011` | cto | 技术总监 |
| `00000000-0000-0000-0000-000000000012` | dev | 开发工程师 |
| `00000000-0000-0000-0000-000000000013` | test | 测试工程师 |
| `00000000-0000-0000-0000-000000000014` | sales | 销售经理 |

### 角色数据

| ID | 角色名称 | 角色代码 | 角色类型 |
|----|---------|---------|---------|
| `00000000-0000-0000-0000-000000000100` | 超级管理员 | admin | 系统内置 |
| `00000000-0000-0000-0000-000000000101` | 普通用户 | user | 普通角色 |
| `00000000-0000-0000-0000-000000000102` | 部门管理员 | dept_admin | 普通角色 |

### 菜单数据

系统包含以下主菜单及其按钮权限：

1. **系统管理** - 目录菜单
2. **用户管理** - 包含新增、编辑、删除、重置密码按钮
3. **角色管理** - 包含新增、编辑、删除、分配权限按钮
4. **菜单管理**
5. **部门管理**
6. **岗位管理**

## 注意事项

### 重要提醒

1. **执行顺序** - 必须严格按照编号顺序执行迁移脚本，后面的脚本依赖前面的表结构
2. **幂等性设计** - 所有 INSERT 语句都使用了 `ON CONFLICT DO NOTHING`，支持重复执行
3. **ID 一致性** - 预定义数据使用固定的 UUID，确保跨环境一致性
4. **外键约束** - 部分表存在外键关联，确保依赖表先被创建

### 数据安全

1. **密码加密** - 所有用户密码都使用 bcrypt 加密存储
2. **敏感信息** - 生产环境请修改初始密码和密钥
3. **审计日志** - 系统会记录所有操作日志

### 环境配置

确保数据库配置正确：

```env
DATABASE_URL=postgresql://username:password@host:port/database
```

### 故障排除

#### 迁移失败

如果迁移执行失败，检查以下内容：

1. UUID 扩展是否启用：
   ```sql
   CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
   ```

2. 依赖表是否存在

3. 是否有重复数据冲突

#### 数据重置

如需重置数据库，执行以下命令：

```sql
-- 删除所有系统表（按依赖顺序）
DROP TABLE IF EXISTS sys_online_user CASCADE;
DROP TABLE IF EXISTS sys_login_log CASCADE;
DROP TABLE IF EXISTS sys_oper_log CASCADE;
DROP TABLE IF EXISTS sys_job_log CASCADE;
DROP TABLE IF EXISTS sys_job CASCADE;
DROP TABLE IF EXISTS sys_notice CASCADE;
DROP TABLE IF EXISTS sys_config CASCADE;
DROP TABLE IF EXISTS sys_dict_data CASCADE;
DROP TABLE IF EXISTS sys_dict_type CASCADE;
DROP TABLE IF EXISTS sys_role_department CASCADE;
DROP TABLE IF EXISTS sys_role_menu CASCADE;
DROP TABLE IF EXISTS sys_user_role CASCADE;
DROP TABLE IF EXISTS sys_menu CASCADE;
DROP TABLE IF EXISTS sys_role CASCADE;
DROP TABLE IF EXISTS sys_user CASCADE;
DROP TABLE IF EXISTS sys_post CASCADE;
DROP TABLE IF EXISTS sys_department CASCADE;

-- 重新执行迁移
```

## 维护说明

- 所有表都包含 `created_at`, `updated_at`, `deleted_at` 字段用于审计
- 使用软删除机制，删除操作不会物理删除数据
- 建议定期备份数据库
