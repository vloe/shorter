ARG APP_NAME=sh-server

# build stage
FROM public.ecr.aws/lambda/provided:al2 as builder

ARG APP_NAME

RUN yum install -y gcc gcc-c++ openssl-devel
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release --bin ${APP_NAME} --features lambda

# runtime stage
FROM public.ecr.aws/lambda/provided:al2

ARG APP_NAME

COPY --from=builder /usr/src/app/target/release/${APP_NAME} ${LAMBDA_RUNTIME_DIR}/bootstrap

CMD ["bootstrap"]
