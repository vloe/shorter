FROM rust:1.78-alpine AS build

ARG APP_NAME=sh-server

WORKDIR /app

# Install build dependencies
RUN apk add --no-cache musl-dev

# copy the source code
COPY . .

# build the application with the lambda feature
RUN cargo build --release --features lambda

# runtime stage
FROM alpine:3.18

ARG APP_NAME=sh-server

# copy the binary from the build stage
COPY --from=build /app/target/release/${APP_NAME} /usr/local/bin/bootstrap

# create data dir 
RUN mkdir /usr/local/bin/data/

# copy the bitmap to data dir
COPY data/domains.bin /usr/local/bin/data

# set the entrypoint to the lambda runtime
ENTRYPOINT ["/usr/local/bin/bootstrap"]
