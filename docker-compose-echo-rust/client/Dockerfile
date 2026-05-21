# Estágio de Compilação
FROM rust:1.80 AS builder

WORKDIR /app
COPY Cargo.toml .
COPY src ./src

RUN cargo build --release

# Estágio de Execução
FROM debian:bookworm-slim

WORKDIR /app
COPY --from=builder /app/target/release/echo-client .

CMD ["./echo-client"]
