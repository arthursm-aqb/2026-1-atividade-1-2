FROM rust:1.80 AS builder
WORKDIR /app
COPY Cargo.toml .
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/echo-server .
EXPOSE 5000
CMD ["./echo-server"]
