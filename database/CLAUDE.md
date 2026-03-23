# Database 开发规范

## Schema 规范
- `backend/migrations/` 下的 SQL 迁移是运行时唯一真源
- `database/schema.prisma` 仅作为参考模型与领域说明
- 不生成 Prisma Client；该文件不作为运行时 ORM 配置
- 表名使用稳定的 `sys_*` 前缀与 snake_case
- 字段必须包含 `created_at` 与 `updated_at`

## 迁移规范
- 每次修改参考 schema 后同步更新迁移
- 迁移文件名保持递增编号与简洁描述，例如 `0001_init_rbac.sql`

## 查询规范
- 运行时代码使用 SQLx
- UUID 由应用层生成或在种子数据中显式提供
- 禁止直接拼接 SQL
- 复杂查询使用参数化 SQL 与显式 JOIN
