FROM rustlang/rust:nightly as builder

WORKDIR /usr/src/app

RUN USER=root cargo init

COPY Cargo.toml .

COPY src src

ADD https://github.com/upx/upx/releases/download/v3.95/upx-3.95-amd64_linux.tar.xz /usr/local

RUN xz -d -c /usr/local/upx-3.95-amd64_linux.tar.xz | tar -xOf - upx-3.95-amd64_linux/upx > /bin/upx && chmod a+x /bin/upx

RUN cargo build --release

RUN upx /usr/src/app/target/release/secrets-injector

FROM debian:stretch-slim
RUN apt-get update && apt-get install openssl -y

COPY --from=builder /usr/src/app/target/release/secrets-injector /opt/

CMD ["./opt/secrets-injector"]
