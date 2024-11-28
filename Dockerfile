# Builder stage
FROM rust:1.75.0-slim-bookworm AS builder

# Create a new empty shell project
WORKDIR /usr/src/mortgagekit-rs

# First copy only Cargo.toml
COPY Cargo.toml ./

# Create dummy Cargo.lock if it doesn't exist
RUN touch Cargo.lock

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

# Set the entrypoint
ENTRYPOINT ["mortgagekit-rs"]
