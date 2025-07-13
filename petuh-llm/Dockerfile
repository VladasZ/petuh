FROM debian:bookworm-slim

# Install OpenSSL runtime and certs
RUN apt update && apt install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY target/x86_64-unknown-linux-gnu/release/petuh-llm /usr/local/bin/petuh-llm
COPY .env /app/.env
WORKDIR /app

RUN chmod +x /usr/local/bin/petuh-llm

ENTRYPOINT ["/usr/local/bin/petuh-llm"]
