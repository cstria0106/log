# BUILD
FROM rust:1-alpine3.13 AS BUILD
WORKDIR /app

RUN rustup component add rustfmt
RUN apk add openssl-dev
RUN apk add build-base
RUN apk add protoc

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --bin log-server \
    && cp target/release/log-server ./log-server

# RUN
FROM alpine:3.13 AS RUN
WORKDIR /app

COPY --from=BUILD /app/log-server .
CMD ["./log-server"]