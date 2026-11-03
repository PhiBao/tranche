.PHONY: install dev-backend dev-frontend build test run reset-demo docker

install:
	cd backend && cargo fetch
	cd frontend && pnpm install

dev-backend:
	cd backend && cargo run

dev-frontend:
	cd frontend && pnpm dev

build:
	cd frontend && pnpm build
	cd backend && cargo build --release

test:
	cd backend && cargo test
	cd frontend && pnpm typecheck

# Build the static frontend and serve it from the Rust binary on :8080.
run: build
	cd backend && FRONTEND_DIR=../frontend/out BIND_ADDR=0.0.0.0:8080 ./target/release/tranche

reset-demo:
	rm -f backend/data/tranche.db backend/data/tranche.db-shm backend/data/tranche.db-wal
	@echo "Demo database cleared. Restart the backend to reseed."

docker:
	docker build -t tranche .
