use std::sync::mpsc::Sender;
use std::net::{TcpListener, TcpStream};

pub struct NetworkManager {
    listener: TcpListener,
    sender: Sender<TcpStream>
}

impl NetworkManager {
    pub fn new(sender: Sender<TcpStream>) -> Self {
        Self {
            listener: std::net::TcpListener::bind("127.0.0.1:8080").unwrap(),
            sender
        }
    }

    pub fn update(&self) {
        while let Ok((stream, _)) = self.listener.accept() {
            self.sender.send(stream).unwrap();
        }
    }
}
