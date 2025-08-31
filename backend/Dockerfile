# Use a base image with Rust and Cargo
FROM rust:latest as builder

WORKDIR /app

# Copy all source files and Cargo files
COPY Cargo.toml .
COPY Cargo.lock .
COPY src ./src

# Build the application
RUN cargo build --release

# Use a minimal base image for the final stage
FROM debian:bookworm-slim

# Install openssl for sqlx
RUN apt-get update && apt-get install -y --no-install-recommends openssl && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/rentinder .

EXPOSE 3000

CMD ["./rentinder"]
