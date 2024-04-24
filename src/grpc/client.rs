use super::hello_world;

use hello_world::greater_client::GreaterClient;
use hello_world::HelloRequest;
use tonic::async_trait;

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
        let mut client = GreaterClient::connect(format!("http://{}", self.address)).await?;

        let request = tonic::Request::new(HelloRequest {
            name: "Self".into(),
        });

        let response = client.say_hello(request).await?;

        println!("RESPONSE={:?}", response);

        Ok(())
    }
}
