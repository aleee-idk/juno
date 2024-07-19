use crate::configuration::Commands;

use super::grpc_juno;
use grpc_juno::juno_services_server::{JunoServices, JunoServicesServer};
use grpc_juno::{EmptyRequest, EmptyResponse, GetFilesRequest, GetFilesResponse, PingResponse};
use std::error::Error;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;
use tonic::transport::Server;
use tonic::{Request, Response, Result, Status};

type Responder<T> = oneshot::Sender<T>;

#[derive(Debug)]
pub struct GrpcServerMessage {
    pub command: Commands,
    pub responder: Responder<()>,
}

#[derive(Debug, Default)]
pub struct GRPCServer {
    transmitter: Option<Sender<GrpcServerMessage>>,
}

impl GRPCServer {
    pub fn new(tx: Sender<GrpcServerMessage>) -> Self {
        Self {
            transmitter: Some(tx),
        }
    }

    async fn send_message(&self, command: Commands) -> Result<(), Box<dyn Error>> {
        let (resp_tx, resp_rx) = oneshot::channel();

        let message = GrpcServerMessage {
            command,
            responder: resp_tx,
        };

        if let Some(tx) = &self.transmitter {
            tx.send(message).await?;

            let response = resp_rx.await?;

            return Ok(response);
        }

        Ok(())
    }

    pub async fn serve(
        address: SocketAddr,
        tx: Sender<GrpcServerMessage>,
    ) -> Result<(), Box<dyn Error>> {
        println!("Starting server on: \"{}\"", address.to_string());

        Server::builder()
            .add_service(JunoServicesServer::new(GRPCServer::new(tx)))
            .serve(address)
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

        let files: Vec<PathBuf> = match self.send_message(Commands::GetFiles { path }).await {
            Ok(()) => vec![],
            Err(_err) => return Err(Status::internal("An internal error has occurred.")),
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
