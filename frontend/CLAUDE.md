# Frontend 开发规范

## 组件规范
- 组件文件：PascalCase（如 UserCard.tsx）
- 组件目录：kebab-case（如 user-card/）
- 组件内部：使用 Arrow Function

## 样式规范
- 使用 Tailwind CSS
- 自定义样式优先使用 Tailwind，必要时补充全局 CSS
- 响应式断点：sm:640px md:768px lg:1024px xl:1280px

## 状态管理
- 组件状态：useState
- 全局状态：Zustand
- 服务端状态：TanStack Query

## API 调用
- 使用 TanStack Query
- 错误处理统一
- 请求封装统一在 `src/lib/http.ts`
- token 从 `src/lib/auth.ts` 读取

## 目录约定
- `src/app/`：入口、providers、router
- `src/routes/`：路由页与测试
- `src/features/`：按领域组织的 API、hooks、类型和导航
- `src/components/`：布局、守卫、通用组件

## 命名规则
- 变量/函数：camelCase
- 常量：UPPER_SNAKE_CASE
- CSS 类：kebab-case

## 路由与启动
- 使用 Vite 作为构建工具
- 路由使用 TanStack Router
- 入口文件：`src/app/main.tsx`
