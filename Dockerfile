# Builder stage
FROM rust:1.75-slim-bullseye as builder

# Create a new empty shell project
WORKDIR /usr/src/mortgagekit-rs

# Copy over manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src/
COPY tests ./tests/

# Build for release with optimizations
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/mortgagekit-rs/target \
    cargo build --release && \
    cp target/release/mortgagekit-rs /usr/local/bin/

# Runtime stage
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /usr/local/bin/mortgagekit-rs /usr/local/bin/mortgagekit-rs

# Set environment variables
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1

# Create a non-root user
RUN useradd -m -U -s /bin/bash mortgagekit

# Switch to non-root user
USER mortgagekit

# Expose the API port
EXPOSE 8080

# Set the entrypoint
ENTRYPOINT ["mortgagekit-rs"]
