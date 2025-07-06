# Build stage
FROM rust:1.87.0 as builder

# Create app directory
WORKDIR /app

# copy over your manifests
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY .env ./.env

COPY .sqlx /app/.sqlx
COPY . /app

RUN cargo build --release

# Final stage
# debian:bookworm-slim
# SSL/TLS (OpenSSL) yang tidak ada secara default di image debian:bookworm-slim.
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/rust-base /usr/local/bin/
EXPOSE 5001

CMD ["/usr/local/bin/rust-base"]