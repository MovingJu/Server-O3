FROM rust:alpine3.23 AS compile_time
RUN apk add --no-cache \
    openssl-dev \
    pkgconfig \
    musl-dev \
    git \
    bash \
    ca-certificates \
    curl \
    && update-ca-certificates
WORKDIR /app

COPY ./Cargo.toml ./Cargo.lock ./
RUN cargo fetch

COPY .env ./
COPY ./migration ./migration
COPY ./src ./src
RUN cargo build --release

FROM alpine:3.23
WORKDIR /app

COPY --from=compile_time /app/target/release/api_movingju_com ./

CMD ["./api_movingju_com"]
