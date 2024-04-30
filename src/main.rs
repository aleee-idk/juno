use std::error::Error;

mod configuration;
mod file_explorer;
mod grpc;

#[tokio::main()]
async fn main() -> Result<(), Box<dyn Error>> {
    let server = grpc::run()?;

    server.connect().await?;

    Ok(())
}
