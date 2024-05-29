use std::error::Error;

use tokio::sync::mpsc;

use crate::player::Player;

use self::configuration::{ConfigMode, CONFIG};
use self::player::PlayerAction;

mod configuration;
mod file_explorer;
mod grpc;
mod player;

async fn init_server() -> Result<(), Box<dyn Error>> {
    let (tx, mut rx) = mpsc::channel::<PlayerAction>(32);

    tokio::spawn(async move {
        let _ = grpc::GRPCServer::serve(tx).await;
    });

    let mut player = Player::new().expect("Error creating player");

    player.handle_message(PlayerAction::Play)?;

    println!("Listening for incomming messages...");

    // this traps the main thread, it should run last.
    while let Some(msg) = rx.recv().await {
        if let Err(err) = player.handle_message(msg) {
            eprintln!("Error handling player action: {}", err);
        }
    }

    Ok(())
}

async fn init_client() -> Result<(), Box<dyn Error>> {
    let client = grpc::GRPCClient::default();
    let _ = client.skip_song().await;
    Ok(())
}

#[tokio::main()]
async fn main() -> Result<(), Box<dyn Error>> {
    match CONFIG.mode {
        ConfigMode::Server => init_server().await?,
        ConfigMode::Client => init_client().await?,
    };

    Ok(())
}
