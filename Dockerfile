# PLAN
FROM lukemathwalker/cargo-chef:0.1.23-alpha.0-rust-1-alpine3.13 as PLAN
WORKDIR /app

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# CACHE
FROM lukemathwalker/cargo-chef:0.1.23-alpha.0-rust-1-alpine3.13 AS CACHE
WORKDIR /app

RUN apk update
RUN apk add openssl-dev
RUN apk add build-base
RUN apk add protoc

COPY --from=PLAN /app/recipe.json recipe.json
RUN cargo chef cook --release --workspace --recipe-path recipe.json

# BUILD
FROM rust:1-alpine3.13 AS BUILD
WORKDIR /app

RUN rustup component add rustfmt
RUN apk add openssl-dev
RUN apk add build-base
RUN apk add protoc

COPY . .
COPY --from=CACHE /app/target target
COPY --from=CACHE $CARGO_HOME $CARGO_HOME

RUN cargo build --release --bin log_server

# RUN
FROM rust:1-alpine3.13 AS RUN
WORKDIR /app

COPY --from=BUILD /app/target/release/log_server /usr/local/bin
ENTRYPOINT ["/usr/local/bin/log_server"]