use std::env;
use std::io;
use std::net;
use std::thread;
use std::time::Duration;

const BUFFER_SIZE: u16 = 1024;
const MAX_CONNECTION_ATTEMPTS: u8 = 15;
const CONNECTION_TIMEOUT_SECS: u8 = 5;

fn main() {
    let host = env::var("ECHO_HOST").unwrap();
    let port = env::var("ECHO_PORT").unwrap();
    let message = env::var("ECHO_MESSAGE").unwrap();
    let addr = format!("{}:{}", host, port);

    for attempt in 1..=MAX_CONNECTION_ATTEMPTS {
        let socket_addr = match addr.to_socket_addrs() {
            Ok(mut addrs) => match addrs.next() {
                Some(a) => a,
                None => {
                    println!("[echo-client] Tentativa {}/{} falhou: DNS sem resultado. Aguardando servidor...", attempt, MAX_CONNECTION_ATTEMPTS);
                    thread::sleep(Duration::from_secs(1));
                    continue;
                }
            },
            Err(e) => {
                println!("[echo-client] Tentativa {}/{} falhou (DNS): {}. Aguardando servidor...",attempt, MAX_CONNECTION_ATTEMPTS, e);
                thread::sleep(Duration::from_secs(1));
                continue;
            }
        };

        // Tenta estabelecer a conexão TCP no endereço resolvido, com timeout definido
        match TcpStream::connect_timeout(&socket_addr, Duration::from_secs(CONNECTION_TIMEOUT_SECS)) {
            // Conexão estabelecida com sucesso; `stream` representa o canal de comunicação
            Ok(mut stream) => {
                // Informa que a conexão com o servidor foi realizada
                println!("[echo-client] Conectado em {}", addr);

                // Envia a mensagem ao servidor codificada em bytes UTF-8
                stream.write_all(message.as_bytes()).expect("Falha ao enviar mensagem");

                // Sinaliza ao servidor que o cliente terminou de enviar dados
                // (equivale ao socket.shutdown(SHUT_WR) do Python)
                // Isso permite que o servidor saiba que não há mais dados a receber
                stream.shutdown(std::net::Shutdown::Write).expect("Falha ao fechar escrita");

                // Vetor que acumula todos os fragmentos da resposta recebida
                let mut response_bytes = Vec::new();
                // Buffer temporário para cada leitura parcial
                let mut buffer = [0u8; BUFFER_SIZE];
                // Laço de leitura: lê a resposta completa do servidor em fragmentos
                loop {
                    // Tenta ler dados do servidor para o buffer
                    match stream.read(&mut buffer) {
                        // Se leu 0 bytes, o servidor encerrou a conexão (resposta completa)
                        Ok(0) => break,
                        // Se leu n bytes, acumula no vetor de resposta
                        Ok(n) => response_bytes.extend_from_slice(&buffer[..n]),
                        // Se ocorreu erro durante a leitura da resposta
                        Err(e) => {
                            // Exibe o erro e encerra o programa com código de falha
                            eprintln!("[echo-client] Erro ao receber resposta: {}", e);
                            std::process::exit(1);
                        }
                    }
                }

                // Converte os bytes recebidos para uma String UTF-8
                // (usa lossy para evitar pânico caso haja bytes inválidos)
                let response = String::from_utf8_lossy(&response_bytes).to_string();
                // Exibe a mensagem enviada
                println!("[echo-client] Enviado:  {}", message);
                // Exibe a resposta recebida do servidor
                println!("[echo-client] Recebido: {}", response);

                // Valida se o eco recebido é idêntico à mensagem enviada
                if response != message {
                    // Se diferente, exibe erro detalhado e encerra com falha
                    eprintln!("[echo-client] Erro: resposta difere da mensagem enviada. Esperado: '{}'. Recebido: '{}'.", message, response);
                    // Termina o processo com código de erro 1
                    std::process::exit(1);
                }

                // Confirma que tudo funcionou corretamente
                println!("[echo-client] Sucesso!");
                // Encerra a função com êxito (conexão é fechada automaticamente ao sair do escopo)
                return;
            }
            // Falha ao conectar ao servidor nesta tentativa
            Err(e) => {
                // Informa a tentativa atual, o erro e que aguardará antes de tentar novamente
                println!("[echo-client] Tentativa {}/{} falhou: {}. Aguardando servidor...",attempt, MAX_CONNECTION_ATTEMPTS, e);
                // Aguarda 1 segundo antes da próxima tentativa (para o servidor ter tempo de subir)
                thread::sleep(Duration::from_secs(1));
            }
        }
    }

    // Se chegou aqui, todas as tentativas falharam
    eprintln!("[echo-client] Não foi possível conectar ao servidor echo.");
    // Encerra o processo com código de erro 1
    std::process::exit(1);
}
