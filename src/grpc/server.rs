use crate::file_explorer;

use super::grpc_juno;
use grpc_juno::juno_request_server::{JunoRequest, JunoRequestServer};
use grpc_juno::{GetFilesRequest, GetFilesResponse, PingRequestMessage, PingResponseMessage};
use std::error::Error;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;
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
impl JunoRequest for GRPCServer {
    async fn ping(
        &self,
        _request: Request<PingRequestMessage>,
    ) -> Result<Response<PingResponseMessage>, Status> {
        let reply = PingResponseMessage {
            message: "pong!".to_string(),
        };

        Ok(Response::new(reply))
    }

    async fn get_files(
        &self,
        request: Request<GetFilesRequest>,
    ) -> Result<Response<GetFilesResponse>, Status> {
        let path = PathBuf::from_str(request.into_inner().path.as_str())
            .expect("Failed to create pathbuf");

        let files = match file_explorer::walk_dir(&path) {
            Ok(files) => files,
            Err(_err) => panic!("Error reading path: {:?}", path),
        };
        eprintln!("DEBUGPRINT[2]: server.rs:44: files={:#?}", files);

        let reply = GetFilesResponse {
            files: files.iter().map(|x| x.display().to_string()).collect(),
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
            .add_service(JunoRequestServer::new(GRPCServer::default()))
            .serve(socket)
            .await?;

        Ok(())
    }
}
