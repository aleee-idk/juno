use core::panic;

use crate::configuration::{Commands, CONFIG};
use crate::grpc::grpc_juno::EmptyRequest;

use super::grpc_juno;

use grpc_juno::juno_services_client::JunoServicesClient;
use grpc_juno::GetFilesRequest;
use tonic::transport::Channel;
use tonic::Request;

#[derive(Debug, Default)]
pub struct GRPCClient {}

impl GRPCClient {
    async fn get_client(&self) -> Result<JunoServicesClient<Channel>, Box<dyn std::error::Error>> {
        let client =
            JunoServicesClient::connect(format!("http://{}", CONFIG.address.to_string())).await?;

        Ok(client)
    }

    pub async fn ping(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut client = self.get_client().await?;

        let request = Request::new(EmptyRequest {});

        let response = client.ping(request).await?.into_inner();

        println!("RESPONSE={:?}", response);

        Ok(())
    }

    pub async fn play(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut client = self.get_client().await?;

        let request = Request::new(EmptyRequest {});

        let response = client.play(request).await?.into_inner();

        println!("RESPONSE={:?}", response);

        Ok(())
    }

    pub async fn pause(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut client = self.get_client().await?;

        let request = Request::new(EmptyRequest {});

        let response = client.pause(request).await?.into_inner();

        println!("RESPONSE={:?}", response);

        Ok(())
    }

    pub async fn play_pause(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut client = self.get_client().await?;

        let request = Request::new(EmptyRequest {});

        let response = client.play_pause(request).await?.into_inner();

        println!("RESPONSE={:?}", response);

        Ok(())
    }

    pub async fn skip_song(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut client = self.get_client().await?;

        let request = Request::new(EmptyRequest {});

        let response = client.skip_song(request).await?.into_inner();

        println!("RESPONSE={:?}", response);

        Ok(())
    }

    pub async fn get_files(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut client = self.get_client().await?;

        if let Commands::GetFiles { path } = &CONFIG.command {
            let request = Request::new(GetFilesRequest {
                path: path.display().to_string(),
            });

            let response = client.get_files(request).await?.into_inner();

            println!("RESPONSE={:?}", response.files);

            return Ok(());
        };

        panic!("Error");
    }
}
