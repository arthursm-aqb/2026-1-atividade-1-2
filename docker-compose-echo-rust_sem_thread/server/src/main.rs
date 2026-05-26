// Importa o módulo de variáveis de ambiente do sistema operacional
use std::env;
// Importa os traits de leitura e escrita de dados (Read e Write)
use std::io::{Read, Write};
// Importa os tipos de rede TCP: ouvinte (TcpListener) e fluxo de conexão (TcpStream)
use std::net::{TcpListener, TcpStream};
// Define o tamanho do buffer de leitura em bytes (1 KB)
const BUFFER_SIZE: usize = 1024;

// Função que trata cada cliente em sua própria thread
// Recebe um TcpStream, que representa a conexão estabelecida com o cliente
fn handle_client(mut stream: TcpStream) {
    // Obtém o endereço IP e porta do cliente conectado
    let addr = stream.peer_addr().unwrap();
    // Exibe no terminal que um novo cliente se conectou
    println!("[echo-server] Conexão de {}", addr);

    // Declara o buffer de leitura preenchido com zeros
    let mut buffer = [0u8; BUFFER_SIZE];

    // Laço principal: continua lendo enquanto houver dados
    loop {
        // Tenta ler dados do cliente para o buffer
        match stream.read(&mut buffer) {
            // Se leu 0 bytes, o cliente encerrou a conexão
            Ok(0) => {
                // Exibe que a conexão foi encerrada pelo cliente
                println!("[echo-server] Conexão encerrada por {}", addr);
                // Sai do laço, encerrando o tratamento desta conexão
                break;
            }
            // Se leu n bytes com sucesso
            Ok(n) => {
                // Fatia apenas os bytes válidos lidos (descarta o resto do buffer)
                let data = &buffer[..n];

                // Tenta enviar de volta ao cliente exatamente os dados recebidos (eco)
                if let Err(e) = stream.write_all(data) {
                    // Se falhar ao enviar, exibe o erro e encerra esta conexão
                    eprintln!("[echo-server] Erro ao enviar para {}: {}", addr, e);
                    // Sai do laço
                    break;
                }
            }
            // Se ocorreu algum erro durante a leitura
            Err(e) => {
                // Exibe o erro no terminal de erro padrão
                eprintln!("[echo-server] Erro na conexão {}: {}", addr, e);
                // Sai do laço encerrando esta conexão
                break;
            }
        }
    }
    // Ao sair do laço, o TcpStream é descartado automaticamente (Drop),
    // o que fecha a conexão TCP com o cliente
}

// Ponto de entrada principal do servidor
fn main() {
    // Lê a variável de ambiente HOST; usa "0.0.0.0" (todas interfaces) como padrão
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    // Lê a variável de ambiente PORT; usa "5000" como padrão
    let port = env::var("PORT").unwrap_or_else(|_| "5000".to_string());
    // Monta o endereço completo no formato "host:porta"
    let addr = format!("{}:{}", host, port);

    // Cria o ouvinte TCP vinculado ao endereço configurado
    // Se falhar (ex: porta ocupada), encerra o programa com mensagem de erro
    let listener = TcpListener::bind(&addr).expect("Falha ao iniciar o servidor");
    // Informa no terminal que o servidor está aguardando conexões
    println!("[echo-server] Escutando em {}", addr);

    // Itera sobre cada nova conexão recebida (bloqueante)
    for stream in listener.incoming() {
        // Verifica se a conexão foi estabelecida com sucesso
        match stream {
            // Conexão aceita com sucesso
            Ok(stream) => {
                // Trata o cliente de forma sequencial, diretamente na thread principal,
                // sem criar novas threads. Enquanto um cliente é atendido,
                // novos clientes aguardam na fila do sistema operacional.
                handle_client(stream);
            }
            // Falha ao aceitar a conexão
            Err(e) => {
                // Exibe o erro mas continua ouvindo por novas conexões
                eprintln!("[echo-server] Erro ao aceitar conexão: {}", e);
            }
        }
    }
}
