FROM rust:1.81 AS builder

WORKDIR /usr/src/app

# Install diesel CLI
RUN cargo install diesel_cli --no-default-features --features postgres

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && \
    echo "fn main() {}" > src/main.rs

RUN cargo build --release

RUN rm -f target/release/deps/evil_lumios*

COPY . .

# Build the application
RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
    libpq-dev \
    libssl-dev \
    postgresql-client \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# Copy diesel CLI from builder
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel

# Copy migration files
COPY --from=builder /usr/src/app/migrations ./migrations

# Copy database configuration
COPY --from=builder /usr/src/app/.env .env

# Copy the built binary
COPY --from=builder /usr/src/app/target/release/evil_lumios .

EXPOSE 8080

# Script to run migrations before starting the application
COPY entrypoint.sh .
RUN chmod +x entrypoint.sh

CMD ["./entrypoint.sh"]