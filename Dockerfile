FROM rust:1.67 as builder
WORKDIR /usr/src/rustservice
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
#RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/local/bin
COPY --from=builder /usr/local/cargo/bin/rustservice .
COPY --from=builder /usr/src/rustservice/config config
CMD ["rustservice"]
