use crate::configuration::{Commands, CONFIG};
use crate::file_explorer;

use super::grpc_juno;
use grpc_juno::juno_services_server::{JunoServices, JunoServicesServer};
use grpc_juno::{EmptyRequest, EmptyResponse, GetFilesRequest, GetFilesResponse, PingResponse};
use std::error::Error;
use std::path::PathBuf;
use std::str::FromStr;
use tokio::sync::mpsc::Sender;
use tonic::transport::Server;
use tonic::{Request, Response, Result, Status};

#[derive(Debug, Default)]
pub struct GRPCServer {
    transmitter: Option<Sender<Commands>>,
}

impl GRPCServer {
    pub fn new(tx: Sender<Commands>) -> Self {
        Self {
            transmitter: Some(tx),
        }
    }

    async fn send_message(&self, message: Commands) -> Result<(), Box<dyn Error>> {
        if let Some(tx) = &self.transmitter {
            tx.send(message).await?;
        }

        Ok(())
    }

    pub async fn serve(tx: Sender<Commands>) -> Result<(), Box<dyn Error>> {
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

        let files = match file_explorer::walk_dir(Some(&path)) {
            Ok(files) => files,
            Err(err) => return Err(Status::invalid_argument(err)),
        };

        let reply = GetFilesResponse {
            files: files.iter().map(|x| x.display().to_string()).collect(),
        };

        Ok(Response::new(reply))
    }

    async fn play(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        if let Err(_err) = self.send_message(Commands::Play).await {
            return Err(Status::internal("An internal error has occurred."));
        }

        Ok(Response::new(EmptyResponse {}))
    }

    async fn pause(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        if let Err(_err) = self.send_message(Commands::Pause).await {
            return Err(Status::internal("An internal error has occurred."));
        }

        Ok(Response::new(EmptyResponse {}))
    }

    async fn play_pause(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        if let Err(_err) = self.send_message(Commands::PlayPause).await {
            return Err(Status::internal("An internal error has occurred."));
        }

        Ok(Response::new(EmptyResponse {}))
    }

    async fn skip_song(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        if let Err(_err) = self.send_message(Commands::SkipSong).await {
            return Err(Status::internal("An internal error has occurred."));
        }

        Ok(Response::new(EmptyResponse {}))
    }
}
