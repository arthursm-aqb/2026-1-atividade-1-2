use std::env;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

const BUFFER_SIZE: usize = 1024;
const MAX_CONNECTION_ATTEMPTS: u32 = 15;
const CONNECTION_TIMEOUT_SECS: u64 = 5;

fn main() {
    let host = env::var("ECHO_HOST").unwrap_or_else(|_| "echo-server".to_string());
    let port = env::var("ECHO_PORT").unwrap_or_else(|_| "5000".to_string());
    let message = env::var("ECHO_MESSAGE").unwrap_or_else(|_| "Olá do cliente echo!".to_string());
    let addr = format!("{}:{}", host, port);

    for attempt in 1..=MAX_CONNECTION_ATTEMPTS {
        match TcpStream::connect_timeout(
            &addr.parse().expect("Endereço inválido"),
            Duration::from_secs(CONNECTION_TIMEOUT_SECS),
        ) {
            Ok(mut stream) => {
                println!("[echo-client] Conectado em {}", addr);

                // Enviar mensagem
                stream
                    .write_all(message.as_bytes())
                    .expect("Falha ao enviar mensagem");

                // Sinalizar fim do envio (equivalente ao shutdown(SHUT_WR))
                stream
                    .shutdown(std::net::Shutdown::Write)
                    .expect("Falha ao fechar escrita");

                // Receber resposta completa
                let mut response_bytes = Vec::new();
                let mut buffer = [0u8; BUFFER_SIZE];
                loop {
                    match stream.read(&mut buffer) {
                        Ok(0) => break,
                        Ok(n) => response_bytes.extend_from_slice(&buffer[..n]),
                        Err(e) => {
                            eprintln!("[echo-client] Erro ao receber resposta: {}", e);
                            std::process::exit(1);
                        }
                    }
                }

                let response = String::from_utf8_lossy(&response_bytes).to_string();
                println!("[echo-client] Enviado:  {}", message);
                println!("[echo-client] Recebido: {}", response);

                if response != message {
                    eprintln!(
                        "[echo-client] Erro: resposta difere da mensagem enviada. Esperado: '{}'. Recebido: '{}'.",
                        message, response
                    );
                    std::process::exit(1);
                }

                println!("[echo-client] Sucesso!");
                return;
            }
            Err(e) => {
                println!(
                    "[echo-client] Tentativa {}/{} falhou: {}. Aguardando servidor...",
                    attempt, MAX_CONNECTION_ATTEMPTS, e
                );
                thread::sleep(Duration::from_secs(1));
            }
        }
    }

    eprintln!("[echo-client] Não foi possível conectar ao servidor echo.");
    std::process::exit(1);
}
