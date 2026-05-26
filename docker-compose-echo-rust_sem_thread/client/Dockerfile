# ── Estágio 1: Compilação ─────────────────────────────────────────────────────
# Usa a imagem oficial do Rust como base para compilar o código-fonte
FROM rust:1.80 AS builder

# Define o diretório de trabalho dentro do container de compilação
WORKDIR /app

# Copia o manifesto do projeto (dependências e metadados)
COPY Cargo.toml .

# Copia o código-fonte do cliente
COPY src ./src

# Compila o projeto em modo release (otimizado, sem símbolos de debug)
RUN cargo build --release

# ── Estágio 2: Execução ───────────────────────────────────────────────────────
# Usa uma imagem Debian mínima para rodar o binário (sem o compilador Rust)
# Isso reduz drasticamente o tamanho final da imagem
FROM debian:bookworm-slim

# Define o diretório de trabalho dentro do container de execução
WORKDIR /app

# Copia apenas o binário compilado do estágio anterior (builder)
COPY --from=builder /app/target/release/echo-client .

# Comando executado ao iniciar o container: roda o cliente
CMD ["./echo-client"]
