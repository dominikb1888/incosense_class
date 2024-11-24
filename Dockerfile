# Stage 1: Build stage
FROM rust:1.72-bullseye as builder

# Set work directory inside the container
WORKDIR /usr/src/app

# Copy Cargo.toml and Cargo.lock for dependency resolution
COPY Cargo.toml Cargo.lock ./

# Pre-build dependencies to cache them for faster builds
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Copy the actual source code
COPY . .

# Build the actual application with static linking
RUN cargo build --release --target x86_64-unknown-linux-musl

# Stage 2: Runtime stage
FROM scratch

# Copy the static binary from the build stage
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/incosense_class /

# Set the binary as the entry point
ENTRYPOINT ["/incosense_class"]

