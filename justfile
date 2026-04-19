default:
    @echo "just start-all / start-db / stop"

build:
    cd backend && mvn clean compile -DskipTests
    cd frontend && bun install

start-db:
    mkdir -p .local/mysql
    mariadbd --datadir $PWD/.local/mysql --pid-file $PWD/.local/mysql/mysql.pid --socket $PWD/.local/mysql/mysql.sock --port 3306 &

start-backend:
    cd backend && mvn spring-boot:run &

start-frontend:
    cd frontend && (test -d node_modules || bun install) && bun run dev &

start-all:
    just start-db
    just start-backend
    just start-frontend

stop:
    -mariadb-admin -u root -p123456 shutdown 2>/dev/null || true
    -pkill -f "spring-boot:run" 2>/dev/null || true
    -pkill -f "vite" 2>/dev/null || true

fmt:
    cd backend && mvn spotless:apply
    cd frontend && bunx prettier --write .
    alejandra flake.nix
