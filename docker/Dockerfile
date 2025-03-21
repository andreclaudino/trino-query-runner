FROM docker.io/rust:1.85-bullseye as builder

ENV RUST_LOG=info
ENV RUST_BACKTRACE=full
ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
    apt-get -y install libssl-dev openssl zlib1g zlib1g-dev libpq-dev cmake protobuf-compiler netcat curl unzip

# create build user with same UID as
RUN adduser -q builder --home /home/builder && \
    mkdir -p /home/builder/workspace

ENV HOME=/home/builder
ENV PATH=$HOME/.cargo/bin:$PATH

# prepare rust
RUN rustup update && \
    rustup component add rustfmt && \
    cargo install cargo-chef --version 0.1.62

WORKDIR /home/builder

ADD src ./src
ADD Cargo.toml ./Cargo.toml
ADD Cargo.lock ./Cargo.lock

RUN cargo build --release

FROM docker.io/ubuntu:24.04

RUN apt-get update && \
    apt-get -y install libssl-dev openssl zlib1g zlib1g-dev libpq-dev protobuf-compiler curl unzip

EXPOSE 8080

ENV RUST_LOG=info

COPY --from=builder /home/builder/target/release/* /usr/bin/
RUN ln /usr/bin/trino-query-runner /usr/bin/run
