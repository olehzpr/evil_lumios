FROM rust:slim

RUN apt-get update && \
    apt-get install -y \
    libpq-dev \
    libssl-dev \
    pkg-config \
    build-essential \
    curl \
    procps

WORKDIR /app

COPY . .

RUN cargo build --release

ENV RUST_BACKTRACE=1

EXPOSE 3000

CMD ["./target/release/evil_lumios"]