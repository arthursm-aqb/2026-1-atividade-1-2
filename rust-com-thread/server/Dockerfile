FROM rust:1.80 AS builder
WORKDIR /app
COPY server.rs .
RUN rustc server.rs -O -o echo-server

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends bash \
  && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/echo-server .
EXPOSE 5000
CMD ["./echo-server"]
