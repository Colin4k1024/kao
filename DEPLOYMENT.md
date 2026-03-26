# Docker部署指南

本文档详细说明如何使用Docker和Docker Compose部署React + Rust企业级后台管理系统。

## 📋 前提条件

- Docker 20.10+
- Docker Compose 2.0+
- 至少2GB可用内存
- 至少10GB可用磁盘空间

## 🚀 快速部署

### 1. 克隆项目

```bash
git clone <repository-url>
cd kao
```

### 2. 配置环境变量

```bash
# 复制环境变量示例文件
cp .env.example .env

# 编辑 .env 文件配置必要的参数
vim .env
```

### 3. 构建并启动服务

```bash
# 使用Docker Compose启动所有服务
docker-compose up -d

# 查看服务状态
docker-compose ps

# 查看日志
docker-compose logs -f
```

### 4. 验证部署

- 前端地址: http://localhost
- 后端地址: http://localhost:8080
- 健康检查: http://localhost:8080/health
- 默认账号: `admin` / `admin123`

## 🏗️ 服务架构

```
┌─────────────────────────────────────────────────────────┐
│                     Docker Network                       │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │   Frontend   │  │   Backend    │  │  PostgreSQL  │  │
│  │   (Nginx)    │──│    (Axum)    │──│   Database   │  │
│  │   Port: 80   │  │  Port: 8080  │  │  Port: 5432  │  │
│  └──────────────┘  └──────────────┘  └──────────────┘  │
└─────────────────────────────────────────────────────────┘
```

## 🔧 服务详情

### PostgreSQL 数据库
- **镜像**: postgres:15-alpine
- **端口**: 5432
- **数据卷**: postgres_data
- **健康检查**: pg_isready
- **初始化**: 自动执行 migrations 目录下的 SQL 文件

### 后端服务 (Rust + Axum)
- **构建方式**: Multi-stage Docker build
- **端口**: 8080
- **健康检查**: GET /health
- **优化**: LTO启用，优化级别3，16个代码生成单元

### 前端服务 (React + Nginx)
- **构建方式**: Multi-stage Docker build
- **端口**: 80
- **SPA路由**: 支持React Router
- **API代理**: /api/* 转发到后端
- **Gzip压缩**: 启用

## 📁 项目结构

```
├── backend/
│   ├── Dockerfile          # 后端Docker配置
│   ├── src/              # Rust源代码
│   └── migrations/       # 数据库迁移脚本
│
├── frontend/
│   ├── Dockerfile         # 前端Docker配置
│   ├── nginx.conf        # Nginx配置
│   ├── src/              # React源代码
│   └── dist/             # 构建产物
│
├── docker-compose.yml     # Docker Compose编排
├── .env                  # 环境变量
└── .env.example         # 环境变量示例
```

## 🔐 安全配置

### 生产环境建议

1. **更改默认密码**
   ```bash
   # 编辑 .env 文件
   DB_PASSWORD=your_secure_password
   JWT_SECRET=your_very_long_random_secret_key
   ```

2. **启用HTTPS**
   ```nginx
   # 修改 nginx.conf
   server {
       listen 443 ssl http2;
       ssl_certificate /path/to/cert.pem;
       ssl_certificate_key /path/to/key.pem;
       # ...
   }
   ```

3. **限制数据库访问**
   ```yaml
   # docker-compose.yml
   postgres:
     networks:
       - kao-network
     # 不要暴露5432端口到主机
   ```

## 📊 日志管理

### 查看日志

```bash
# 查看所有服务日志
docker-compose logs

# 查看特定服务日志
docker-compose logs -f backend
docker-compose logs -f frontend
docker-compose logs -f postgres

# 查看最近100行日志
docker-compose logs --tail 100
```

### 日志轮转

```bash
# 配置日志轮转（/etc/docker/daemon.json）
{
  "log-driver": "json-file",
  "log-opts": {
    "max-size": "10m",
    "max-file": "3"
  }
}
```

## 🔄 更新部署

### 重新构建镜像

```bash
# 拉取最新代码
git pull

# 重新构建并启动
docker-compose up -d --build

# 只重建某个服务
docker-compose up -d --build backend
```

### 数据库迁移

```bash
# 连接到数据库容器
docker-compose exec postgres psql -U postgres -d kao_db

# 执行迁移
docker-compose exec backend ./kao-backend migrate
```

## 🛠️ 故障排除

### 服务无法启动

```bash
# 检查端口占用
lsof -i :80
lsof -i :8080
lsof -i :5432

# 检查Docker状态
docker system df
docker-compose ps

# 查看详细日志
docker-compose logs --tail=100
```

### 数据库连接失败

```bash
# 检查数据库容器
docker-compose exec postgres psql -U postgres -d kao_db

# 检查连接字符串
docker-compose exec backend env | grep DATABASE_URL

# 重启数据库
docker-compose restart postgres
```

### 前端无法访问API

```bash
# 检查后端服务
curl http://localhost:8080/health

# 检查Nginx配置
docker-compose exec frontend nginx -t

# 查看Nginx日志
docker-compose logs frontend
```

## 🧹 清理环境

### 停止服务

```bash
docker-compose down
```

### 删除所有数据（谨慎操作）

```bash
docker-compose down -v
docker system prune -a
```

### 保留数据卷重启

```bash
docker-compose down
docker-compose up -d
```

## 📈 性能优化

### 后端优化

1. **增加连接池大小**
   ```bash
   # .env
   DATABASE_MAX_CONNECTIONS=20
   DATABASE_MIN_CONNECTIONS=5
   ```

2. **启用缓存**
   ```bash
   # .env
   REDIS_URL=redis://redis:6379
   ```

### 前端优化

1. **CDN加速静态资源**
2. **启用浏览器缓存**
3. **压缩JavaScript和CSS**

## 🔍 监控

### 查看资源使用

```bash
# Docker stats
docker stats

# 查看特定容器
docker stats kao-backend
```

### 健康检查

```bash
# 检查所有服务健康状态
curl -f http://localhost:8080/health && echo "Backend: OK"
curl -f http://localhost/ && echo "Frontend: OK"
```

## 📞 技术支持

如遇问题，请：
1. 查看日志: `docker-compose logs`
2. 检查配置: `docker-compose config`
3. 提交Issue

## 📄 许可证

MIT License
