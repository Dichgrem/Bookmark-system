# 部署指南

## Docker（推荐）

```bash
git clone <repo-url> subnav
cd subnav
docker compose up --build -d
# 打开 http://localhost:8989
```

数据持久化在 `./data/` 目录，挂载为容器内 `/app/.local/`。

### Docker Compose 配置

仓库自带 `docker-compose.yml`（无需手写），直接 `docker compose up --build -d` 即可：

```yaml
services:
  bookmark:
    image: brantcoat/subnav:latest
    ports:
      - "8989:8989"
    environment:
      - DATABASE_PATH=/app/.local/bookmark.db
    volumes:
      - ./data:/app/.local
    restart: unless-stopped
```

- 数据库显式指向容器内 `/app/.local/bookmark.db`，配合 `./data` 卷持久化
- 前端静态文件由镜像内置（`FRONTEND_DIR=/frontend/dist`），无需挂载

### Docker 镜像结构

三阶段构建：

| 阶段 | 基础镜像 | 用途 |
|------|----------|------|
| 1 | node:22-alpine | 安装 pnpm 依赖，`vite build` 输出 `/app/dist` |
| 2 | rust:1-alpine + musl-dev | 编译后端为 musl 静态二进制 |
| 3 | scratch | 拷贝静态二进制 + 前端 dist，最终镜像 ~15MB |

生产环境中 Rust 二进制是唯一运行进程：
- 通过 `ServeDir` 提供前端静态文件（`FRONTEND_DIR` 环境变量）
- 处理 `/user/*`、`/category/*`、`/bookmark/*` API 请求
- 前端 Axios 使用同源 `baseURL`，无需独立反向代理

## 环境变量

| 变量 | 默认值 | 说明 |
|------|--------|------|
| `PORT` | `8989` | 服务端口 |
| `DATABASE_PATH` | `.local/bookmark.db` | SQLite 数据库路径（相对后端进程工作目录；Docker 中固定为 `/app/.local/bookmark.db`） |
| `CORS_ORIGIN` | `http://localhost:5173` | CORS 允许源（开发环境） |
| `JWT_EXPIRE_HOURS` | `72` | JWT 过期时间（小时） |
| `JWT_SECRET` | （自动生成） | 手动指定 JWT 密钥；未设置时读取数据库 `secrets` 表，仍无则首启随机生成 32 字节并持久化 |
| `FRONTEND_DIR` | `../frontend/dist` | 前端构建产物目录（相对后端进程工作目录；Docker 中覆盖为 `/frontend/dist`） |
| `RUST_LOG` | （无） | 日志级别（如 `info,bookmark_backend=debug`） |

## 常用 Docker 命令

```bash
docker compose up --build -d        # 构建并启动
docker compose down                  # 停止
docker compose logs -f               # 查看日志
docker compose up --build -d --force-recreate  # 强制重建
```

也可以通过 just 运行：

```bash
just docker-up       # 构建并启动
just docker-down     # 停止
just docker-logs     # 查看日志
just docker-rebuild  # 强制重建
```
