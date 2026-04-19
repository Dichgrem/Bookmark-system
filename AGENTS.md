# Agent Instructions for Bookmark-system

This is a full-stack Java Spring Boot and Vue 3 application.

## System Architecture
- **Backend (`/backend`)**: Java 17, Spring Boot, Maven, MyBatis Plus, JWT. 
- **Frontend (`/frontend`)**: Vue 3, Vite, Element Plus, Axios. Uses `bun` for package management.
- **Database**: MariaDB (MySQL compatible).

## Development Environment & Task Runner
- **Environment dependencies**: Managed via Nix. If you need Java or MariaDB, they are provided by the Nix dev shell (`flake.nix`).
- **Task Runner**: Uses `just` (see `justfile`). **Prefer `just` commands over manual shell scripts.**

### Key Commands
- `just start-all` - Starts the database, backend, and frontend concurrently.
- `just stop` - Gracefully shuts down the DB, Spring Boot, and Vite processes.
- `just start-db` - Initializes and starts a local MariaDB instance inside `.local/mysql`.
- `just build` - Compiles the backend (`mvn clean compile -DskipTests`) and installs frontend dependencies (`bun install`).

## Important Notes for Agents
- **Database Location**: The database runs locally on port `3306` with data stored in `$PWD/.local/mysql`. Do not assume a system-level MySQL service.
- **Monorepo structure**: Ensure you are in the correct directory (`backend/` for Maven commands, `frontend/` for bun/Vite commands) if not using `just`. 
- **Backend framework**: The backend relies on `lombok` and `mybatis-plus`. Ensure any added entities use Lombok annotations and follow MyBatis Plus conventions.
