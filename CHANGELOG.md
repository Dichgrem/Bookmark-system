# Changelog

All notable changes to subnav.

## [0.2.0] - 2026-07-21

### Added
- Fuzzy search with pinyin support (Fuse.js + pinyin-pro)
- Backend and frontend test suites (11 Rust tests, 7 Vitest tests)
- `r2d2` connection pool with transactions and DB indexes
- Parallel bookmark/category loading on startup
- Server timeout (Tower middleware)

### Changed
- Element Plus switched to on-demand imports (unplugin-vue-components)
- `pinyin-pro` lazy-loaded for smaller initial bundle
- Main bundle code-split into separate chunks
- Drag delay reduced by 50ms
- HTML import rewritten with line-by-line parser

### Fixed
- `list_categories` and `list_bookmarks_raw` unwrap → `?` error propagation
- Dead `ALTER TABLE` hacks and stale CSS/docs removed
- Docker DB path mismatch resolved, `rusqlite::Transaction` API used
- Vue Router `next()` replaced with `return` pattern

### Internal
- `useBookmarks` and `useDragDrop` composables extracted from HomeView
- Justfile: `fmt/check/fix` merged into `lint` and `fmt`
- Docs restructured into 6 bilingual Chinese files

## [0.1.0] - 2026-06-05

### Added
- User system: registration, login, JWT auth, admin role (first registrant)
- Category management: CRUD, tree view with collapse/expand, drag-to-sort
- Bookmark management: CRUD, drag-to-sort with undo, category assignment
- JSON import/export
- HTML bookmarks import/export (Netscape format, Chrome/Firefox/Edge compatible)
- Favicon auto-fetch and favicon column in bookmark list
- Full-text search across title and URL
- i18n support (zh-CN / English)
- Dark/light theme toggle
- Password change dialog for all users
- Admin panel: add/delete users
- Keyboard shortcuts: `Ctrl+K` focus search, `Escape` close dialogs
- Nix flake development environment with `just` task runner
- Docker multi-stage build (musl + scratch, ~15MB final image)
- Docker Compose for one-command deployment

### Changed
- Backend: Java/Spring/MariaDB → Rust/Axum/SQLite rewrite
- Frontend: Vue 2 → Vue 3 Composition API + Element Plus
- JWT secret auto-generated on first startup (no config required)
- Passwords hashed with bcrypt
- Input validation on all endpoints
