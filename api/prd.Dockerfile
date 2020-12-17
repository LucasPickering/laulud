# Actual version doesn't matter here since we use a nightly one
FROM rust:slim AS builder

WORKDIR /app
RUN apt-get update && \
    apt-get install -y \
    libssl-dev \
    pkg-config \
    && \
    rm -rf /var/lib/apt/lists/*
COPY rust-toolchain Cargo.toml Cargo.lock ./
COPY ./src/ ./src/
RUN cargo build --release

FROM debian:buster-slim
WORKDIR /app
RUN apt-get update && \
    apt-get install -y \
    ca-certificates \
    libssl1.1 \
    && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/laulud-api .
COPY entrypoint.sh ./Rocket.toml ./
ENTRYPOINT ["/app/entrypoint.sh"]
CMD ["/app/laulud-api"]
