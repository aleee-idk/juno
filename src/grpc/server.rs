use super::hello_world;
use hello_world::greater_server::{Greater, GreaterServer};
use hello_world::{HelloRequest, HelloResponse};
use std::error::Error;
use std::net::SocketAddr;
use tonic::transport::Server;
use tonic::{async_trait, Request, Response, Result, Status};

#[derive(Debug, Default)]
pub struct GRPCServer {
    address: String,
}

impl GRPCServer {
    pub fn new(address: String) -> Self {
        Self { address }
    }
}

#[tonic::async_trait]
impl Greater for GRPCServer {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        println!("Got a request {:?}", request);

        let reply = hello_world::HelloResponse {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

#[async_trait]
impl super::Connection for GRPCServer {
    async fn connect(&self) -> Result<(), Box<dyn Error>> {
        println!("Starting server on: \"{}\"", self.address);

        let socket: SocketAddr = self.address.parse()?;

        Server::builder()
            .add_service(GreaterServer::new(GRPCServer::default()))
            .serve(socket)
            .await?;

        Ok(())
    }
}
