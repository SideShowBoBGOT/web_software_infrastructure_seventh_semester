FROM rust:1.76-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update
RUN apt-get install -y libssl3 ca-certificates

COPY --from=builder /app/target/release/lab_5_frontend ./server

COPY static ./static
COPY .env .

ARG PORT
ENV PORT=${PORT}
EXPOSE ${PORT}

CMD ["./server"]