FROM rust:latest as builder

WORKDIR /usr/src/app

RUN USER=root cargo init

COPY Cargo.toml .

RUN cargo build --release

COPY src src

RUN cargo build --release

FROM debian:stretch-slim
COPY --from=builder /usr/src/app/target/release/secrets-injector /opt/
CMD ["./opt/secrets-injector"]