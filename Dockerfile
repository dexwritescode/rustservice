FROM rust:1.73-bookworm as builder

WORKDIR /rustservice
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12 AS runtime
WORKDIR /app
COPY --from=builder /rustservice/config /app/config
COPY --from=builder /rustservice/target/release/rustservice /app/rustservice
ENTRYPOINT ["/app/rustservice"]
