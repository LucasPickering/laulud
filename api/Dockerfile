# Actual version doesn't matter here since we use a nightly one
FROM rust:slim

WORKDIR /app
RUN apt-get update && \
    apt-get install -y \
    libssl-dev \
    pkg-config \
    && \
    rm -rf /var/lib/apt/lists/*
COPY rust-toolchain .
RUN cargo install cargo-watch
# If you add stuff here, you may need to update the prd.Dockerfile too
