FROM rodrigodealer/rust-nightly-musl:latest as builder

WORKDIR /home/rust/

RUN USER=root cargo init

COPY Cargo.toml .

COPY src src

RUN cargo build --release --target=x86_64-unknown-linux-musl

USER root

ADD https://github.com/upx/upx/releases/download/v3.95/upx-3.95-amd64_linux.tar.xz /usr/local

RUN xz -d -c /usr/local/upx-3.95-amd64_linux.tar.xz | tar -xOf - upx-3.95-amd64_linux/upx > /bin/upx && chmod a+x /bin/upx

RUN upx /home/rust/target/x86_64-unknown-linux-musl/release/secrets-injector

FROM scratch
COPY --from=builder /home/rust/target/x86_64-unknown-linux-musl/release/secrets-injector /opt/
CMD ["./opt/secrets-injector"]
