use super::grpc_juno;

use grpc_juno::juno_request_client::JunoRequestClient;
use grpc_juno::GetFilesRequest;
use tonic::async_trait;
use tonic::Request;

#[derive(Debug, Default)]
pub struct GRPCClient {
    address: String,
}

impl GRPCClient {
    pub fn new(address: String) -> Self {
        Self { address }
    }
}

#[async_trait]
impl super::Connection for GRPCClient {
    async fn connect(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut client = JunoRequestClient::connect(format!("http://{}", self.address)).await?;

        let request = Request::new(GetFilesRequest {
            path: "/home/aleidk/Documents/".to_string(),
        });

        let response = client.get_files(request).await?.into_inner();

        println!("RESPONSE={:?}", response.files);

        Ok(())
    }
}
