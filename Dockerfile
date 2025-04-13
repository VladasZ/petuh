FROM debian:bookworm-slim

# Install OpenSSL runtime and certs
RUN apt update && apt install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY target/x86_64-unknown-linux-gnu/release/glavpetuh /usr/local/bin/glavpetuh
COPY .env /app/.env
WORKDIR /app

RUN chmod +x /usr/local/bin/glavpetuh

ENTRYPOINT ["/usr/local/bin/glavpetuh"]
