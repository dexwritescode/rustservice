FROM rust:1.73-bookworm as builder

WORKDIR /todoservice
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12 AS runtime
WORKDIR /app
COPY --from=builder /todoservice/config /app/config
COPY --from=builder /todoservice/target/release/todoservice /app/todoservice
ENTRYPOINT ["/app/todoservice"]
