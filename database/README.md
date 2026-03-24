# Database Schema

This directory contains the database schema definitions for the RuoYi-style admin framework.

## Migration Strategy

The database uses a migration-first approach with SQL files in the `backend/migrations/` directory. The current Prisma schema is kept for reference but the actual database schema is defined by the SQL migrations.

## Current Schema

The schema implements a Role-Based Access Control (RBAC) system with data scope enforcement:

- `sys_users`: User accounts with department associations
- `sys_departments`: Hierarchical department structure with ancestor paths
- `sys_roles`: Roles with data scope policies (ALL, CUSTOM, DEPT, DEPT_AND_CHILD, SELF)
- `sys_menus`: Menu items with permissions and hierarchical structure
- Junction tables for many-to-many relationships:
  - `sys_user_roles`: User-role assignments
  - `sys_role_menus`: Role-menu permissions
  - `sys_role_departments`: Custom data scope assignments

## Migrations

1. `0001_init_rbac.sql`: Creates all RBAC tables with appropriate constraints and indexes
2. `0002_seed_admin.sql`: Seeds initial admin user, role, and menu data

## Data Scope Implementation

The system implements RuoYi-style data scope filtering:
- ALL: Access to all records
- CUSTOM: Access to selected departments
- DEPT: Access to own department only
- DEPT_AND_CHILD: Access to own department and sub-departments
- SELF: Access to own records only