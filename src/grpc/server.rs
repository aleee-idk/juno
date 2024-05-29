use crate::configuration::CONFIG;
use crate::{file_explorer, PlayerAction};

use super::grpc_juno;
use grpc_juno::juno_services_server::{JunoServices, JunoServicesServer};
use grpc_juno::{EmptyRequest, GetFilesRequest, GetFilesResponse, PingResponse, StatusResponse};
use std::error::Error;
use std::path::PathBuf;
use std::str::FromStr;
use tokio::sync::mpsc::Sender;
use tonic::transport::Server;
use tonic::{async_trait, Request, Response, Result, Status};

#[derive(Debug, Default)]
pub struct GRPCServer {
    transmitter: Option<Sender<PlayerAction>>,
}

impl GRPCServer {
    pub fn new(tx: Sender<PlayerAction>) -> Self {
        Self {
            transmitter: Some(tx),
        }
    }

    async fn send_message(&self, message: PlayerAction) -> Result<(), Box<dyn Error>> {
        if let Some(tx) = &self.transmitter {
            tx.send(message).await?;
        }

        Ok(())
    }

    pub async fn serve(tx: Sender<PlayerAction>) -> Result<(), Box<dyn Error>> {
        println!("Starting server on: \"{}\"", CONFIG.address.to_string());

        Server::builder()
            .add_service(JunoServicesServer::new(GRPCServer::new(tx)))
            .serve(CONFIG.address)
            .await?;

        Ok(())
    }
}

#[tonic::async_trait]
impl JunoServices for GRPCServer {
    async fn ping(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<PingResponse>, Status> {
        let reply = PingResponse {
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
            Err(err) => return Err(Status::invalid_argument(err)),
        };

        let reply = GetFilesResponse {
            files: files.iter().map(|x| x.display().to_string()).collect(),
        };

        Ok(Response::new(reply))
    }

    async fn skip_song(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<StatusResponse>, Status> {
        if let Err(_err) = self.send_message(PlayerAction::SkipSong).await {
            return Err(Status::internal("An internal error has occurred."));
        }

        Ok(Response::new(StatusResponse {}))
    }
}

#[async_trait]
impl super::Connection for GRPCServer {
    async fn connect(&self) -> Result<(), Box<dyn Error>> {
        println!("Starting server on: \"{}\"", CONFIG.address.to_string());

        Server::builder()
            .add_service(JunoServicesServer::new(GRPCServer::default()))
            .serve(CONFIG.address)
            .await?;

        Ok(())
    }
}
