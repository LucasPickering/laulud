# Actual version doesn't matter here since we use a nightly one
FROM rust:slim

WORKDIR /app
COPY rust-toolchain .
RUN cargo install cargo-watch
