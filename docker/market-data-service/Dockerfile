# 1. Build Stage
FROM rust:1.97.1 AS builder
WORKDIR /usr/src/app
COPY ./market-data-service/ .
COPY ./shared /usr/src/shared
RUN apt-get update && apt-get install -y --no-install-recommends \
    protobuf-compiler \
    && cargo generate-lockfile && cargo build --release

# 2. Runtime Stage
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/market-data-service .
RUN useradd -m appuser && chown -R appuser /app
USER appuser
CMD ["./market-data-service"]
