FROM rust:1.82-slim as builder
WORKDIR /usr/src/app
RUN cargo new --bin server
WORKDIR /usr/src/app/server
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
RUN rm src/*.rs
COPY src ./src
RUN rm ./target/release/deps/server*
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && \
    apt-get install -y libssl3 ca-certificates && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/server/target/release/server ./server

# We don't copy static files here anymore, they'll come from the volume
EXPOSE ${FRONTEND_PORT}
CMD ["./server"]