FROM 192.168.0.201:30500/petuh-base:0.1.0

COPY target/x86_64-unknown-linux-gnu/release/petuh-llm /usr/local/bin/petuh-llm
COPY .env /app/.env
WORKDIR /app

RUN chmod +x /usr/local/bin/petuh-llm

ENTRYPOINT ["/usr/local/bin/petuh-llm"]
