FROM rust:latest as builder
WORKDIR /usr/src
COPY src ./src
COPY schema ./schema
COPY static ./static
COPY Cargo.toml Cargo.lock build.rs ./
RUN apt-get update -yqq; apt-get install -yqq --no-install-recommends capnproto
RUN cargo test
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get clean; apt-get update && apt-get install -y bash ca-certificates openssl curl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/nachtwacht-rust /usr/local/bin/nachtwacht-rust
WORKDIR /n8w8
COPY static ./static
ENV RUST_LOG=info
USER 1000
EXPOSE 8080
CMD ["/usr/local/bin/nachtwacht-rust"]
