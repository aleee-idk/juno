use std::error::Error;
use std::net::{SocketAddr, TcpListener};

use tonic::async_trait;

use self::client::GRPCClient;
use self::server::GRPCServer;

mod client;
mod server;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[async_trait]
pub trait Connection {
    async fn connect(&self) -> Result<(), Box<dyn Error>>;
}

fn is_socket_in_use(addr: String) -> bool {
    let socket: SocketAddr = addr.parse().expect("Failed to create socket");
    match TcpListener::bind(socket) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn run() -> Result<Box<dyn Connection>, Box<dyn Error>> {
    let addr = "[::1]:50051";

    if is_socket_in_use(addr.to_string()) {
        Ok(Box::new(GRPCServer::new(addr.to_string())))
    } else {
        Ok(Box::new(GRPCClient::new(addr.to_string())))
    }
}
