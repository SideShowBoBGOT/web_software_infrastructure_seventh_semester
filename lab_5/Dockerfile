# Builder stage
FROM rust:1.76-slim as builder

WORKDIR /app
COPY . .

RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

WORKDIR /app

# Copy the built executable
COPY --from=builder /app/target/release/your_binary_name ./server
# Copy static files
COPY static ./static
# Copy .env file
COPY .env .

# Expose the port using the value from .env
ARG PORT
ENV PORT=${PORT}
EXPOSE ${PORT}

CMD ["./server"]