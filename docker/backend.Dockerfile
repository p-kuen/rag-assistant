# Backend Dockerfile - Rust mit Multi-Stage Build
FROM rust:1.75-slim AS builder

WORKDIR /app

# Dependencies für Build
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Dummy Build für Dependencies Caching
RUN cargo init --name rag-backend-orchestrator
COPY Cargo.toml Cargo.lock* ./
RUN cargo build --release || true
RUN rm -rf src

# Source Code kopieren und bauen
COPY src ./src
RUN cargo build --release

# Runtime Stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Binary vom Builder kopieren
COPY --from=builder /app/target/release/rag-backend-orchestrator /app/

# User für Security
RUN useradd -m -u 1000 appuser && chown -R appuser:appuser /app
USER appuser

EXPOSE 8080

CMD ["./rag-backend-orchestrator"]
