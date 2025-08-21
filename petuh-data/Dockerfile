FROM 192.168.0.201:30500/petuh-base:0.1.0

COPY target/x86_64-unknown-linux-gnu/release/petuh-data /usr/local/bin/petuh-data
COPY .env /app/.env
COPY ./petuh-data/migrations /app/../migrations
WORKDIR /app

RUN chmod +x /usr/local/bin/petuh-data

ENTRYPOINT ["/usr/local/bin/petuh-data"]
