# Multi-stage Docker build for Rust Axum application
# Optimized for Fly.io deployment

# Build stage
FROM rust:1.75-alpine AS builder

# Install dependencies for building
RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    openssl-libs-static \
    pkgconfig \
    git

# Set environment variables for static linking
ENV RUSTFLAGS="-C target-feature=-crt-static"
ENV PKG_CONFIG_ALL_STATIC=1
ENV PKG_CONFIG_ALL_FEATURE_LOWER_CASE=1

# Create app user
RUN addgroup -g 1000 app && adduser -D -s /bin/sh -u 1000 -G app app

# Create app directory
WORKDIR /app

# Copy dependency files
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release && rm -rf src target/release/deps/rust*

# Copy source code
COPY src ./src
COPY migrations ./migrations

# Build the application
RUN cargo build --release

# Runtime stage
FROM alpine:3.19 AS runtime

# Install runtime dependencies
RUN apk add --no-cache \
    ca-certificates \
    libgcc \
    openssl

# Create app user
RUN addgroup -g 1000 app && adduser -D -s /bin/sh -u 1000 -G app app

# Create app directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/rust-base-1 /app/rust-base
COPY --from=builder /app/migrations /app/migrations

# Change ownership to app user
RUN chown -R app:app /app

# Switch to app user
USER app

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:3000/health || exit 1

# Run the application
CMD ["./rust-base"]