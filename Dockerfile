# Multi-stage Dockerfile for Astronaut server
# Build stage
FROM rust:1-bookworm AS builder
WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

# Build only the server crate in release mode
# Note: package name remains `astronaut-server`; directory is `crates/server`
RUN cargo build --release -p astronaut-server

# Runtime stage
FROM debian:bookworm-slim AS runtime
WORKDIR /app

# Minimal runtime deps
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy binary
COPY --from=builder /app/target/release/astronaut-server /usr/local/bin/astronaut-server

ENV RUST_LOG=info
EXPOSE 8080

CMD ["/usr/local/bin/astronaut-server"]
