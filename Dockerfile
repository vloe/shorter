# build stage
FROM rust:1.80.1 as builder

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release --bin sh-server --features lambda

# runtime stage
FROM debian:bullseye-slim

# install necessary libs for it to work w lambda
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/sh-server /usr/local/bin/bootstrap

ENTRYPOINT ["/usr/local/bin/bootstrap"]
