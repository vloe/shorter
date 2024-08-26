ARG RUST_VERSION=1.80.1
ARG APP_NAME=sh-server

# build stage
FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
RUN apk add --no-cache musl-dev openssl-dev
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release --bin ${APP_NAME} --features lambda

# runtime stage
FROM public.ecr.aws/lambda/provided:al2
ARG APP_NAME
COPY --from=build /usr/src/app/target/release/${APP_NAME} ${LAMBDA_RUNTIME_DIR}/bootstrap
CMD ["bootstrap"]
