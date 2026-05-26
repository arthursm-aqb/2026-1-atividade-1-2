// Importa o módulo de variáveis de ambiente do sistema operacional
use std::env;
// Importa os traits de leitura e escrita de dados (Read e Write)
use std::io::{Read, Write};
// Importa TcpStream para criar a conexão TCP com o servidor
// Importa ToSocketAddrs para resolver nomes de host (DNS) em endereços de rede
use std::net::{TcpStream, ToSocketAddrs};
// Importa o módulo de threads para usar sleep entre tentativas
use std::thread;
// Importa o tipo Duration para representar intervalos de tempo
use std::time::Duration;

// Tamanho do buffer de leitura em bytes (1 KB)
const BUFFER_SIZE: u16 = 1024;
// Número máximo de tentativas de conexão ao servidor antes de desistir
const MAX_CONNECTION_ATTEMPTS: u8 = 15;
// Tempo máximo em segundos para aguardar a conexão ser estabelecida
const CONNECTION_TIMEOUT_SECS: u8 = 5;

// Ponto de entrada principal do cliente
fn main() {
    // Lê a variável de ambiente ECHO_HOST; usa "echo-server" (nome do serviço Docker) como padrão
    let host = env::var("ECHO_HOST").unwrap();
    // Lê a variável de ambiente ECHO_PORT; usa "5000" como padrão
    let port = env::var("ECHO_PORT").unwrap();
    // Lê a mensagem a ser enviada ao servidor a partir da variável de ambiente
    let message = env::var("ECHO_MESSAGE").unwrap();
    // Monta o endereço completo no formato "host:porta"
    let addr = format!("{}:{}", host, port);

    // Laço de tentativas: tenta conectar ao servidor até MAX_CONNECTION_ATTEMPTS vezes
    for attempt in 1..=MAX_CONNECTION_ATTEMPTS {
        // Resolve o hostname para um SocketAddr usando DNS (necessário porque Docker usa
        // nomes de serviço como "echo-server-rust" — não IPs literais)
        // to_socket_addrs() retorna um iterador de endereços resolvidos
        let socket_addr = match addr.to_socket_addrs() {
            // Pega o primeiro endereço IP resolvido para o hostname
            Ok(mut addrs) => match addrs.next() {
                Some(a) => a,
                // Nenhum endereço encontrado: DNS ainda não conhece o host (servidor não subiu)
                None => {
                    println!(
                        "[echo-client] Tentativa {}/{} falhou: DNS sem resultado. Aguardando servidor...",
                        attempt, MAX_CONNECTION_ATTEMPTS
                    );
                    thread::sleep(Duration::from_secs(1));
                    // Passa para a próxima tentativa do laço
                    continue;
                }
            },
            // Falha na resolução DNS (ex: host ainda não existe na rede Docker)
            Err(e) => {
                println!(
                    "[echo-client] Tentativa {}/{} falhou (DNS): {}. Aguardando servidor...",
                    attempt, MAX_CONNECTION_ATTEMPTS, e
                );
                thread::sleep(Duration::from_secs(1));
                // Passa para a próxima tentativa do laço
                continue;
            }
        };

        // Tenta estabelecer a conexão TCP no endereço resolvido, com timeout definido
        match TcpStream::connect_timeout(
            // Usa o SocketAddr já resolvido (IP real) — não mais um hostname
            &socket_addr,
            // Define o tempo máximo de espera pela conexão
            Duration::from_secs(CONNECTION_TIMEOUT_SECS),
        ) {
            // Conexão estabelecida com sucesso; `stream` representa o canal de comunicação
            Ok(mut stream) => {
                // Informa que a conexão com o servidor foi realizada
                println!("[echo-client] Conectado em {}", addr);

                // Envia a mensagem ao servidor codificada em bytes UTF-8
                stream
                    .write_all(message.as_bytes())
                    .expect("Falha ao enviar mensagem");

                // Sinaliza ao servidor que o cliente terminou de enviar dados
                // (equivale ao socket.shutdown(SHUT_WR) do Python)
                // Isso permite que o servidor saiba que não há mais dados a receber
                stream
                    .shutdown(std::net::Shutdown::Write)
                    .expect("Falha ao fechar escrita");

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
                    eprintln!(
                        "[echo-client] Erro: resposta difere da mensagem enviada. Esperado: '{}'. Recebido: '{}'.",
                        message, response
                    );
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
                println!(
                    "[echo-client] Tentativa {}/{} falhou: {}. Aguardando servidor...",
                    attempt, MAX_CONNECTION_ATTEMPTS, e
                );
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
