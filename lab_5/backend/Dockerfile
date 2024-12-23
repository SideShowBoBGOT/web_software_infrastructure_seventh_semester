FROM rust:1.82-slim as builder

WORKDIR /usr/src/app

# Create a new empty shell project and move into it
RUN cargo new --bin server
WORKDIR /usr/src/app/server

# Copy only the dependency files first
COPY Cargo.toml Cargo.lock ./

# Build dependencies only (this layer will be cached if dependencies don't change)
RUN cargo build --release
RUN rm src/*.rs

# Now copy your actual source code
COPY src ./src

# Build the application (will only rebuild your code, not dependencies)
RUN rm ./target/release/deps/server*
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies in a single RUN to reduce layers
RUN apt-get update && \
    apt-get install -y libssl3 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy only the built binary
COPY --from=builder /usr/src/app/server/target/release/server ./server

EXPOSE ${BACKEND_PORT}
CMD ["./server"]