FROM debian:bookworm-slim

# Install OpenSSL runtime and certs
RUN apt update && apt install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY target/x86_64-unknown-linux-gnu/release/petuh-data /usr/local/bin/petuh-data
COPY .env /app/.env
COPY ./petuh-data/migrations /app/../migrations
WORKDIR /app

RUN chmod +x /usr/local/bin/petuh-data

ENTRYPOINT ["/usr/local/bin/petuh-data"]
