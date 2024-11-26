# Stage 1: Build stage
FROM rust:1.76-bullseye as builder

ENV CARGO_TARGET_AARCH_64_UNKNOWN_LINUX_MUSL_LINKER=aarch64-linux-gnu-gcc
ENV CC=aarch64-linux-gnu-gcc

RUN apt-get update && apt-get install -y \
    musl-tools \
    && apt-get clean

ENV SQLX_OFFLINE true
ENV OPENSSL_STATIC=1
ENV RUSTFLASGS="-C target-feature=-crt-static"
ENV CARGO_TARGET=aarch64-unknown-linux-musl

# Set work directory inside the container
WORKDIR /usr/src/app

# Copy Cargo.toml and Cargo.lock for dependency resolution
COPY Cargo.toml Cargo.lock ./

# Pre-build dependencies to cache them for faster builds
RUN rustup target add aarch64-unknown-linux-musl
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo check --release

# Copy the actual source code
COPY . .

# Build the actual application with static linking
RUN cargo build --release --target aarch64-unknown-linux-musl

# Stage 2: Runtime stage
FROM scratch

# Copy the static binary from the build stage
COPY --from=builder /usr/src/app/target/aarch64-unknown-linux-musl/release/incosense_class /
COPY --from=builder /usr/src/app/configuration.yaml /

# Set the binary as the entry point
ENTRYPOINT ["/incosense_class"]
