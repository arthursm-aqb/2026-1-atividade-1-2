use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

const BUFFER_SIZE: usize = 1024;

fn handle_client(mut stream: TcpStream) {
    let addr = stream.peer_addr().unwrap();
    println!("[echo-server] Conexão de {}", addr);

    let mut buffer = [0u8; BUFFER_SIZE];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("[echo-server] Conexão encerrada por {}", addr);
                break;
            }
            Ok(n) => {
                let data = &buffer[..n];

                if let Err(e) = stream.write_all(data) {
                    eprintln!("[echo-server] Erro ao enviar para {}: {}", addr, e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("[echo-server] Erro na conexão {}: {}", addr, e);
                break;
            }
        }
    }
}

fn main() {
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "5000".to_string());
    let addr = format!("{}:{}", host, port);

    let listener = TcpListener::bind(&addr).expect("Falha ao iniciar o servidor");
    println!("[echo-server] Escutando em {}", addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("[echo-server] Erro ao aceitar conexão: {}", e);
            }
        }
    }
}
