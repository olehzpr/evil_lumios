FROM rust:1.82-slim-bullseye AS builder

RUN apt-get update && \
    apt-get install -y \
    libpq-dev \
    libssl-dev \
    pkg-config \
    build-essential \
    curl

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && \
    echo "fn main() {}" > src/main.rs

RUN cargo build --release

COPY src ./src
COPY migrations ./migrations

RUN cargo build --release

RUN cargo install diesel_cli --no-default-features --features postgres

FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get install -y \
    libpq5 \
    openssl \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/evil_lumios /app/evil_lumios

COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel
COPY --from=builder /app/migrations /app/migrations

COPY docker-entrypoint.sh /app/
RUN chmod +x /app/docker-entrypoint.sh

EXPOSE 3000
EXPOSE 5432
EXPOSE 6379

ENTRYPOINT ["/app/docker-entrypoint.sh"]