FROM debian:buster-slim
EXPOSE 8080
RUN apt-get clean; apt-get update && apt-get install -y bash ca-certificates openssl curl && rm -rf /var/lib/apt/lists/*
COPY target/debug/nachtwacht-fe /usr/local/bin/
WORKDIR /n8w8
ENV RUST_LOG actix_web=info
USER 1000
COPY static ./static
CMD ["/usr/local/bin/nachtwacht-fe"]
