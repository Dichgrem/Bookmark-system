# Bookmark System development commands

default:
    @just --list

# ─────────────────────────────────────────────
#  Build & Server
# ─────────────────────────────────────────────

build:
    cd backend && cargo build
    cd frontend && pnpm install

start-backend:
    cd backend && CORS_ORIGIN=http://localhost:5173 ALLOW_PRIVATE_URLS=true cargo run &

start-frontend:
    cd frontend && pnpm install && pnpm dev &

start-all: start-backend start-frontend

stop:
    -pkill -f "bookmark-backend"
    -pkill -f "vite"

# ─────────────────────────────────────────────
#  Lint & Format
# ─────────────────────────────────────────────

# read-only checks (clippy + prettier --check). CI should run this.
lint:
    cd backend && cargo clippy
    cd frontend && pnpx prettier --check .

# auto-fix everything (fmt + clippy --fix + prettier --write). Run before commit.
fmt:
    cd backend && cargo fmt && cargo clippy --fix --allow-dirty
    cd frontend && pnpx prettier --write .
    alejandra flake.nix

# ─────────────────────────────────────────────
#  Docker
# ─────────────────────────────────────────────

docker-up:
    docker compose up --build -d

docker-down:
    docker compose down

docker-logs:
    docker compose logs -f

docker-rebuild:
    docker compose up --build -d --force-recreate

# ─────────────────────────────────────────────
#  Tests
# ─────────────────────────────────────────────

test: test-backend test-frontend

test-backend:
    cd backend && cargo test

test-frontend:
    cd frontend && pnpm test
