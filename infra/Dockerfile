FROM debian:bookworm-slim

# Install OpenSSL runtime and certs
RUN apt update && apt install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

