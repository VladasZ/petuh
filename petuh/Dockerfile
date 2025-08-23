FROM 192.168.0.201:30500/petuh-base:0.1.0

COPY target/x86_64-unknown-linux-gnu/release/petuh /usr/local/bin/petuh
COPY .env /app/.env
WORKDIR /app

ENV PRODUCTION=true

RUN chmod +x /usr/local/bin/petuh

ENTRYPOINT ["/usr/local/bin/petuh"]
