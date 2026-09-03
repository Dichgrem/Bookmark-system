# 项目结构

```
subnav/
├── backend/               # Rust 后端
│   ├── Cargo.toml         # 依赖声明（axum / rusqlite / r2d2 / reqwest ...）
│   ├── Cargo.lock
│   └── src/
│       ├── main.rs        # 入口：路由注册（auth_routes + api_routes）、CORS、超时、静态文件、启动
│       ├── config.rs      # 环境变量配置（PORT, DATABASE_PATH, CORS_ORIGIN, FRONTEND_DIR ...）
│       ├── db.rs          # SQLite 初始化：rusqlite_migration 版本化迁移 + r2d2 连接池 + JWT 密钥生成（secrets 表）
│       ├── auth.rs        # JWT 签发/验证 + AuthUser + AdminUser 提取器
│       ├── error.rs       # AppError 统一错误（rusqlite/r2d2/serde/JoinError 等自动转换）
│       ├── result.rs      # ApiResult<T> 统一返回体 {code, msg, data}
│       ├── models.rs      # User / Category / Bookmark 结构体（camelCase 序列化）
│       └── handlers/
│           ├── mod.rs
│           ├── user.rs        # 登录/注册/创建用户/删除用户/修改密码/用户列表
│           ├── category.rs    # 分类 CRUD + 树形递归删除 + 归属校验 + 批量排序（≤1000）
│           ├── bookmark.rs    # 书签 CRUD + 归属校验 + 图标抓取 + 批量排序（≤1000）
│           ├── check.rs       # 链接检测：并发 HTTP 探测 + 进度状态 + 24h 缓存
│           ├── export.rs      # HTML 导出（Netscape 格式渲染）+ favicon URL 提取
│           └── import.rs      # HTML 导入（行级解析 + 同名分类复用 + 去重）
├── frontend/              # Vue 3 前端
│   ├── index.html
│   ├── package.json
│   ├── pnpm-lock.yaml
│   ├── vite.config.js     # Vite + @vitejs/plugin-vue + Element Plus 按需引入 + Vitest 配置
│   ├── README.md
│   ├── public/favicon.svg
│   └── src/
│       ├── main.js        # Vue 应用入口（挂载 router + i18n）
│       ├── App.vue        # 根组件：仅 <router-view />
│       ├── style.css      # 全局样式 + 暗色模式 CSS 变量
│       ├── router/        # Vue Router（JWT 登录守卫：未登录跳 /login，已登录访问 /login 跳首页）
│       ├── utils/
│       │   ├── request.js # Axios 实例（开发环境 baseURL :8989，自动注入 Bearer token）
│       │   ├── constants.js # localStorage 键名常量（token/user/暗色/语言/展开状态）
│       │   └── tree.js    # 扁平列表 → 嵌套分类树构建（buildTree）
│       ├── i18n/          # vue-i18n 中英文翻译
│       │   ├── index.js   # createI18n（读取本地保存的语言，默认 zh-CN）
│       │   ├── zh-CN.js
│       │   └── en-US.js
│       ├── composables/
│       │   ├── useBookmarks.js   # 书签/分类状态管理 + API 调用 + 删除/撤销
│       │   ├── useDragDrop.js    # 拖拽排序逻辑（书签网格 + 分类树）
│       │   ├── useImportExport.js # 导入/导出 HTML 处理
│       │   ├── useScroll.js      # 内容区滚动：回到顶部 + 分类滚动定位
│       │   └── useUserAdmin.js   # 管理员用户管理（列表/添加/删除对话框）
│       ├── components/
│       │   ├── Sidebar.vue      # 左侧分类树 + 齿轮菜单（导入/导出/获取图标/链接检测/语言/主题/账号）
│       │   ├── SearchBar.vue    # 搜索框
│       │   ├── BookmarkGrid.vue # 书签卡片网格
│       │   ├── EditDialog.vue   # 新增/编辑书签对话框
│       │   ├── CheckLinks.vue   # 链接检测弹窗（进度 + 分组结果 + 标记正常/跳转编辑）
│       │   ├── DialogFooter.vue # 弹窗统一底部按钮（确定/取消）
│       │   ├── PixelPacman.vue  # 吃豆人加载动画
│       │   └── Login.vue        # 登录/注册表单（含动画背景）
│       ├── views/
│       │   ├── HomeView.vue     # 主界面容器：侧边栏 + 搜索 + 网格 + 各弹窗 + 动画 + 主题
│       │   └── LoginView.vue    # 登录页容器
│       └── __tests__/           # 前端测试（Vitest）
│           ├── router.test.js   # 路由守卫（3 个用例）
│           └── tree.test.js     # buildTree 树构建（5 个用例）
├── docs/                  # 文档
│   ├── structure_zh-CN.md # 项目结构
│   ├── deploy_zh-CN.md    # 部署指南
│   ├── usage_zh-CN.md     # 使用手册
│   ├── api_zh-CN.md       # API 参考
│   ├── dev-guide_zh-CN.md # 开发指南
│   └── testing_zh-CN.md   # 测试指南
├── justfile               # 任务运行器（build / start-* / lint / fmt / test / docker-*）
├── flake.nix / flake.lock # Nix 开发环境
├── Dockerfile             # 三阶段构建（node 构建前端 → rust-musl 编译后端 → scratch 运行）
├── docker-compose.yml
├── CHANGELOG.md
├── LICENSE
└── README.md
```

## 技术栈

| 层 | 技术 | 说明 |
|---|---|---|
| 后端框架 | Rust + Axum 0.7 | Tokio 异步运行时 |
| 数据库 | SQLite (rusqlite 0.31, bundled) | 编译自带 SQLite，无需系统安装 |
| 连接池 | r2d2 0.8 + r2d2_sqlite 0.24 | 池大小 4，配合 WAL 与外键约束 |
| 数据库迁移 | rusqlite_migration 1 | 版本化迁移（建表 + 索引），兼容旧库 ALTER |
| 认证 | JWT (jsonwebtoken 9) | Bearer Token，密钥首启自动生成或由 `JWT_SECRET` 注入 |
| 密码加密 | bcrypt 0.16 | 纯 Rust 实现 |
| 链接检测 | reqwest 0.12 (rustls-tls) | 并发 20 的 HEAD 探测，GET 回退，24h 进程内缓存 |
| 前端框架 | Vue 3 (Composition API, `<script setup>`) | 纯 JS |
| UI 组件库 | Element Plus 2 | 按需引入（unplugin-vue-components），组件自动导入 |
| 模糊搜索 | Fuse.js 7 + pinyin-pro 3 | 标题/网址模糊匹配，支持拼音，pinyin-pro 懒加载 |
| HTTP 客户端 | Axios | 拦截器自动附加 `Authorization: Bearer <token>` |
| 构建工具 | Vite 8 | 开发服务器 + 生产构建（代码分割） |
| 国际化 | vue-i18n 9 | 中英文切换（本地持久化） |
| 路由 | vue-router 4 | JWT 登录守卫 |
| 日志 | tracing + tracing-subscriber | 可通过 `RUST_LOG` 控制 |
| 后端测试 | Rust 内置 `#[test]` | 14 个单元测试，零额外依赖 |
| 前端测试 | Vitest 4 + @vue/test-utils + happy-dom | 8 个单元测试 |
| 开发环境 | Nix + just | 声明式依赖 + 任务运行器 |
| 格式化 | rustfmt + Prettier + Alejandra | 分别格式化 Rust/JS/Nix |
| 部署 | Docker (musl + scratch) | 静态编译，最终镜像 ~15MB |
