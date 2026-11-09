# syntax=docker/dockerfile:1

FROM node:22-slim AS frontend
WORKDIR /app/frontend
RUN corepack enable
COPY frontend/package.json frontend/pnpm-lock.yaml ./
RUN corepack prepare pnpm@11.1.1 --activate && pnpm install --frozen-lockfile
COPY frontend ./
RUN pnpm build

FROM rust:1-slim-bookworm AS backend
WORKDIR /app
COPY backend/Cargo.toml backend/Cargo.lock ./
RUN mkdir src && printf 'fn main() {}' > src/main.rs
RUN cargo build --release
RUN rm -rf src
COPY backend/src ./src
RUN touch src/main.rs && cargo build --release

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl jq \
    && curl -fsSL https://cli.gokite.ai/install.sh | bash \
    && rm -rf /var/lib/apt/lists/*
ENV PATH="/root/.local/bin:${PATH}"
WORKDIR /app
COPY --from=backend /app/target/release/tranche ./tranche
COPY --from=frontend /app/frontend/out ./frontend/out
COPY scripts ./scripts
RUN chmod +x ./scripts/kite_payout.sh ./scripts/kite_status.sh ./scripts/entry.sh && mkdir -p /data
ENV BIND_ADDR=0.0.0.0:8080 \
    DATABASE_URL=sqlite:///data/tranche.db?mode=rwc \
    DATABASE_MAX_CONNECTIONS=5 \
    FRONTEND_DIR=/app/frontend/out \
    PUBLIC_BASE_URL=http://localhost:8080 \
    REPPO_MODE=auto \
    KITE_MODE=ledger \
    ALLOW_MEDIUM_ASSURANCE=true \
    ALLOW_FIXTURE_RELEASE=false
EXPOSE 8080
VOLUME ["/data"]
ENTRYPOINT ["/app/scripts/entry.sh"]
