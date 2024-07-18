use std::net::{SocketAddr, TcpListener};


pub use self::client::GRPCClient;
pub use self::server::GRPCServer;

mod client;
mod server;

pub mod grpc_juno {
    tonic::include_proto!("juno");
}

/// Return true if the addr is already in use, false otherwise
pub fn is_socket_in_use(addr: SocketAddr) -> bool {
    match TcpListener::bind(addr) {
        Ok(_) => false,
        Err(_) => true,
    }
}
