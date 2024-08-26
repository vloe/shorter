# build stage
FROM rust:1.80.1 as builder

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release --bin sh-server --features lambda

# runtime stage
FROM debian:bullseye-slim

COPY --from=builder /usr/src/app/target/release/sh-server /usr/local/bin/bootstrap

ENTRYPOINT ["/usr/local/bin/bootstrap"]
