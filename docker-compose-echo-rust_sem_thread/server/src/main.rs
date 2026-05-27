use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{thread, time};

const RESPONSE: &str = "mensagem de resposta do servidor";
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
                let response_received = String::from_utf8_lossy(&data);
                println!("[echo-server] Mensagem de {}: {}", addr, response_received);
                
                thread::sleep(time::Duration::from_secs(1));

                println!("[echo-server] Enviando de volta para {}: {}", addr, RESPONSE);

                if let Err(e) = stream.write_all(RESPONSE.as_bytes()) {
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
    let host = env::var("HOST").unwrap();
    let port = env::var("PORT").unwrap();
    let addr = format!("{}:{}", host, port);

    let listener = TcpListener::bind(&addr).expect("Falha ao iniciar o servidor");

    println!("[echo-server] Escutando em {}", addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream),
            Err(e) => eprintln!("[echo-server] Erro ao aceitar conexão: {}", e),
        }
    }
}
