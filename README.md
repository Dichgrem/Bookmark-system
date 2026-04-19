# 智能书签管理系统 (Smart Bookmark System)

## 📌 项目简介
本项目是一个前后端彻底分离的**智能书签管理系统**，项目旨在解决用户在多设备、多平台间难以统一管理和检索书签的痛点。我们不仅提供基础的书签增删改查功能，还将引入**AI 大模型技术**（如自动摘要、智能分类）以及**云原生分布式架构**（缓存、微服务、消息队列等），打造一个高可用、智能化的现代化 Web 应用。

## 🛠️ 技术栈 (Tech Stack)
- **前端 (Frontend)**: Vue 3, Vite, Element Plus, Axios
- **后端 (Backend)**: Java 17, Spring Boot, MyBatis Plus, Maven, JWT, Spring Security
- **数据库 (Database)**: MariaDB (MySQL 兼容)
- **进阶中间件 (Middleware)**: Redis, RabbitMQ (规划中)
- **开发与部署**: Nix 环境隔离, Just (任务运行器), Docker (规划中)

## 🚀 快速启动 (Quick Start - Linux / macOS)

本项目使用 `Nix` 和 `just` 简化了本地开发环境的配置和任务运行。

```bash
# 1. 进入开发环境 (需要安装 Nix，它会自动提供 Java 和 MariaDB 环境)
nix develop

# 2. 启动所有服务 (数据库、后端 Spring Boot、前端 Vite)
just start-all

# 3. 停止所有服务
just stop

# 其他常用命令：
just build         # 编译后端并安装前端 bun 依赖
just start-db      # 仅启动本地数据库 (数据文件储存在当前目录的 .local/mysql 中)
just start-backend # 仅启动后端
just start-frontend# 仅启动前端
just fmt           # 全栈统一格式化代码 (包含后端 Java、前端 Vue/TS 和 Nix 脚本)
```

---

## 🪟 Windows 开发环境搭建指南

由于本项目重度使用了 Nix 隔离本地数据库和 Java 环境，对于 Windows 开发者，我们提供以下两种环境配置方式：

### 方案一：使用 WSL2 (强烈推荐 ⭐)
在 Windows 下获得与 Linux 完美一致的零配置开发体验。
1. 打开 PowerShell 以管理员身份运行：`wsl --install` 安装 Ubuntu。
2. 重启后进入 WSL Ubuntu 终端，执行官方安装脚本安装 Nix：
   `sh <(curl -L https://nixos.org/nix/install) --daemon`
3. 在 WSL 终端中进入项目目录，直接按照上方的 **【快速启动】** 步骤运行 `nix develop` 和 `just start-all` 即可。

### 方案二：Windows 原生环境 (需手动安装依赖)
如果你不想使用 WSL，需要手动在 Windows 系统中准备以下底层环境：
1. **安装 JDK 17**：并配置好 `JAVA_HOME` 环境变量。
2. **安装 Bun**：在 PowerShell 运行 `powershell -c "irm bun.sh/install.ps1 | iex"` 安装前端包管理器。
3. **安装 Just**：建议使用 Scoop (`scoop install just`) 安装任务运行器。
4. **手动配置数据库**：
   - 原生 Windows 下 `just start-db` 将无法运行。
   - 请自行下载安装 [MariaDB Windows 版](https://mariadb.org/download/) 或 MySQL，默认监听 3306 端口。
   - 确保本地数据库的账号为 `root`，密码为 `root`（或者前往 `backend/src/main/resources/application.yml` 中修改为你自己的密码）。
5. **分步启动项目**：
   - 打开一个终端运行：`just start-backend`
   - 打开另一个终端运行：`just start-frontend`

---

## 🎯 小组目标与代办规划


### 阶段一：夯实基础功能
- [ ] **纯洁架构 (10分)**: 搭建 Vue 3 + Spring Boot 前后端分离骨架，严禁出现 JSP/Thymeleaf。
- [ ] **数据持久化 (15分)**: 构思至少 4 张核心业务表（例如：`用户表`、`书签表`、`分类/目录表`、`标签表`），并实现复杂的关联查询（一对多、多对多）。
- [ ] **分层规范 (10分)**: 后端严格落实 `Controller -> Service -> Mapper` 的标准系统架构，职责清晰。
- [ ] **RESTful API (15分)**: 产出至少 6 个核心业务接口，前后端必须使用 JSON 对接，包含统一返回体（`Result` 对象）。
- [ ] **工程化基建 (10分)**: 规范 Maven 依赖管理，制定清晰的 Git 提交流，并开始编写团队协作和接口文档。

### 阶段二：工程素养与安全防护
- [ ] **RBAC 权限控制 (10分)**: 引入 Spring Security + JWT，实现基于角色的访问控制（普通用户只能看自己的书签，管理员可以管理系统）。
- [ ] **团队协作与规范 (10分)**: 保持按周期的固定 Git 团队协作记录，统一注解规范，使用 Swagger/Knife4j 生成自动化的 API 接口文档，并在开发中灵活应用 AI Coding 技术。

### 阶段三：AI 大模型赋能业务
- [ ] **基础 AI 辅助 (10分)**: 接入第三方大模型 API，当用户提交书签链接时，后台自动抓取网页内容并生成**一句话摘要**或**智能翻译**。
- [ ] **高阶 AI 工作流 (20分)**: 运用 Spring AI / LangChain4j，或者接入 Dify 工作流，实现更复杂的逻辑（如：给书签**自动打标签分类**，或结合私有知识库增强检索）。

### 阶段四：云端架构与高并发应对
- [ ] **Redis 缓存加速 (10分)**: 引入 Redis，将会话数据、高频访问的字典表（如公共书签分类）加载到内存，显著降低数据库压力。
- [ ] **Docker 容器化部署 (10分)**: 编写 Dockerfile 和 docker-compose，实现云端（前端、后端、数据库）的一键丝滑启动部署。
- [ ] *(选做)* **消息队列异步处理 (15分)**: 引入 RabbitMQ，将耗时操作（如生成包含数千条书签的长篇导出报告、发送注册邮件）转为异步队列处理。
- [ ] *(选做)* **微服务拆分实践 (20分)**: 尝试将单体架构拆分为 3 个独立微服务（如：用户中心、书签核心、AI 处理），并使用 Nacos 进行简单的服务注册与发现。
