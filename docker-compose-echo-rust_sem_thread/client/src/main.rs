use std::env;
use std::io::{Read, Write};
use std::net::TcpStream;

const BUFFER_SIZE: usize = 1024;

fn main() {
    let host = env::var("ECHO_HOST").unwrap();
    let port = env::var("ECHO_PORT").unwrap();
    let message = env::var("ECHO_MESSAGE").unwrap();
    let addr = format!("{}:{}", host, port);

    let mut stream = TcpStream::connect(&addr).unwrap();
    println!("[echo-client] Conectado em {}", addr);

    stream.write_all(message.as_bytes()).expect("Falha ao enviar mensagem");
    stream.shutdown(std::net::Shutdown::Write).expect("Falha ao fechar escrita");

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

    let response = String::from_utf8_lossy(&response_bytes);
    println!("[echo-client] Enviado:  {}", message);
    println!("[echo-client] Recebido: {}", response);

    println!("[echo-client] fim de execução!");
}

