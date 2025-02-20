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

EXPOSE 3000

CMD ["./evil_lumios"]
