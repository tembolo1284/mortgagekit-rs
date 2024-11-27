# Builder stage
FROM rust:1.75.0-slim-bookworm AS builder

# Create a new empty shell project
WORKDIR /usr/src/mortgagekit-rs

# Copy over manifests
COPY Cargo.toml Cargo.lock ./

# Create src directory and copy source files
COPY src ./src/
COPY tests ./tests/

# Build for release
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /usr/src/mortgagekit-rs/target/release/mortgagekit-rs /usr/local/bin/

# Set environment variables
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1

# Create a non-root user
RUN useradd -m -U -s /bin/bash mortgagekit

# Switch to non-root user
USER mortgagekit

# Expose the API port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/api/v1/health || exit 1

# Set the entrypoint
ENTRYPOINT ["mortgagekit-rs"]
