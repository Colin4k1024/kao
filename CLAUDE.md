# AI Coding Project - 核心规则

## 安全
- 所有 API 必须包含认证
- 禁止直接拼接 SQL，使用 SQLx 参数化查询
- 敏感数据使用环境变量
- 所有输入必须验证

## 编码规范
- TypeScript 严格模式
- ESLint + Prettier
- 组件命名：PascalCase
- 函数命名：camelCase
- Hooks 命名：useXxx

## 项目结构
```
├── frontend/     # Vite + React 前端
├── backend/      # Rust + Axum 后端
├── database/     # SQL migrations + Prisma 风格参考模型
├── .agent/      # AI Agent 配置
└── tests/       # 测试文件
```

## AI 工作流
1. 风险分级检查
2. 构建上下文（加载项目代码、技术栈知识）
3. 生成执行计划
4. Agent Loop 执行
5. 自动化测试验证
6. 提交部署
