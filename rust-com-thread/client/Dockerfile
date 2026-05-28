FROM rust:1.80 AS builder
WORKDIR /app
COPY client.rs .
RUN rustc client.rs -O -o echo-client

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/echo-client .
CMD ["./echo-client"]
