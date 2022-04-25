# Actual version doesn't matter here since we use a nightly one
FROM rust:slim AS builder
WORKDIR /app
RUN apt-get update && \
    apt-get install -y \
    musl-tools \
    musl-dev \
    pkg-config \
    && \
    rm -rf /var/lib/apt/lists/*
COPY . .
# *After* copying in files, so we have rust-toolchain.toml
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

# Copy binary into a minimal runtime image
FROM alpine:latest
WORKDIR /app
RUN apk add --no-cache ca-certificates
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/laulud-api .
COPY ./Rocket.toml ./
CMD ["/app/laulud-api"]
