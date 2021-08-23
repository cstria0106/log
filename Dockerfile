# BUILD
FROM rust:1-alpine3.13 AS BUILD
WORKDIR /app

RUN rustup component add rustfmt
RUN apk add openssl-dev
RUN apk add build-base
RUN apk add protoc
RUN apk add git

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    RUSTFLAGS="-C target-feature=-crt-static" cargo build --release --bin log-server \
    && cp target/release/log-server ./log-server

# RUN
FROM alpine:3.13 AS RUN
WORKDIR /app

RUN apk add libgcc

COPY --from=BUILD /app/log-server .

EXPOSE ${PORT}
CMD ["./log-server"]
