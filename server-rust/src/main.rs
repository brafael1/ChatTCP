use log::{error, info, warn};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

struct ChatServer {
    clients: Arc<Mutex<HashMap<String, TcpStream>>>,
}

impl ChatServer {
    fn new() -> Self {
        ChatServer {
            clients: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn broadcast(&self, message: &str, exclude_username: Option<&str>) {
        let clients = self.clients.lock().unwrap();
        for (username, mut stream) in clients.iter() {
            if let Some(exclude) = exclude_username {
                if username == exclude {
                    continue;
                }
            }
            if let Err(e) = stream.write_all(format!("{}\n", message).as_bytes()) {
                warn!("Erro ao enviar para {}: {}", username, e);
            }
        }
    }

    fn remove_client(&self, username: &str) {
        let mut clients = self.clients.lock().unwrap();
        if clients.remove(username).is_some() {
            info!("Cliente '{}' desconectado", username);
            drop(clients);
            self.broadcast(&format!("{} saiu do chat", username), None);
        }
    }

    fn handle_client(&self, stream: TcpStream, username: String) {
        let addr = stream.peer_addr().unwrap();
        info!("Novo cliente conectado: {} ({})", username, addr);

        self.broadcast(&format!("{} entrou no chat", username), Some(&username));

        let reader = BufReader::new(stream.try_clone().unwrap());
        let mut lines = reader.lines();

        while let Some(Ok(line)) = lines.next() {
            let message = line.trim();
            if message.is_empty() {
                continue;
            }

            if message == "/quit" {
                break;
            }

            info!("{}: {}", username, message);
            self.broadcast(&format!("{}: {}", username, message), None);
        }

        self.remove_client(&username);
    }

    fn accept_connections(&self, listener: &TcpListener) {
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let addr = stream.peer_addr().unwrap();
                    info!("Conexão recebida de: {}", addr);

                    let username = Self::get_username(stream.try_clone().unwrap());
                    let clients = Arc::clone(&self.clients);

                    {
                        let mut clients_lock = clients.lock().unwrap();
                        if clients_lock.contains_key(&username) {
                            let _ = stream.write_all(b"Erro: usuario ja conectado\n");
                            continue;
                        }
                        clients_lock.insert(username.clone(), stream.try_clone().unwrap());
                    }

                    let server = Self {
                        clients: Arc::clone(&self.clients),
                    };
                    let username_clone = username.clone();
                    thread::spawn(move || {
                        server.handle_client(stream, username_clone);
                    });
                }
                Err(e) => {
                    error!("Erro ao aceitar conexão: {}", e);
                }
            }
        }
    }

    fn get_username(stream: TcpStream) -> String {
        if let Some(Ok(line)) = BufReader::new(stream.try_clone().unwrap()).lines().next() {
            let _ = stream.try_clone().unwrap().write_all(b"OK\n");
            return line.trim().to_string();
        }
        format!("User{}", rand_id())
    }
}

fn rand_id() -> u16 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos() as u16;
    nanos
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_secs()
        .init();

    info!("===========================================");
    info!("       TCP Chat Server - Rust");
    info!("===========================================");
    info!("Iniciando servidor na porta 8080...");

    let listener = match TcpListener::bind("0.0.0.0:8080") {
        Ok(l) => {
            info!("Servidor ouvindo em 0.0.0.0:8080");
            l
        }
        Err(e) => {
            error!("Falha ao iniciar servidor: {}", e);
            std::process::exit(1);
        }
    };

    let server = ChatServer::new();
    server.accept_connections(&listener);
}
