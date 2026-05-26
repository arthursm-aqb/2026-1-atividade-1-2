# ── Estágio 1: Compilação ─────────────────────────────────────────────────────
# Usa a imagem oficial do Rust como base para compilar o código-fonte
FROM rust:1.80 AS builder

# Define o diretório de trabalho dentro do container de compilação
WORKDIR /app

# Copia o manifesto do projeto (dependências e metadados)
COPY Cargo.toml .

# Copia o código-fonte do servidor
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
COPY --from=builder /app/target/release/echo-server .

# Declara que o container expõe a porta 5000 (documentação para o Docker)
EXPOSE 5000

# Comando executado ao iniciar o container: roda o servidor
CMD ["./echo-server"]
