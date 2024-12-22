FROM rust:1.76-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update
RUN apt-get install -y libssl3 ca-certificates

COPY --from=builder /app/target/release/server ./server

# COPY static ./static

EXPOSE ${BACKEND_PORT}

CMD ["./server"]