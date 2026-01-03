# --- Stage 1: Build ---
FROM rust:1.92-slim AS builder

WORKDIR /usr/src/byc
COPY . .

# Build the release binary
RUN cargo build --release

# --- Stage 2: Runtime ---
FROM debian:bookworm-slim

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /usr/src/byc/target/release/byc /usr/local/bin/byc

# Copy your mock data folder into the container
COPY ./data ./data

# Set the entrypoint so you can pass arguments easily
ENTRYPOINT ["byc"]