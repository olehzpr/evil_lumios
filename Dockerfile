FROM rust:1.82 AS builder

RUN apt-get update && \
    apt-get install -y \
    libpq-dev \
    libssl-dev \
    pkg-config \
    build-essential \
    curl

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && \
  apt-get install -y \
  libpq5 \
  openssl \
  ca-certificates && \
  rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/evil_lumios /app/
COPY --from=builder /app/inline-config.toml /app/

RUN chmod +x /app/evil_lumios

EXPOSE 3000

ENTRYPOINT ["/app/evil_lumios"]
