use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;

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

    println!("Listening for incomming messages...");

    // This macro will wait on multiple futures and will return when the first one resolves
    tokio::select! {
        Some(msg) = rx.recv() => {
            if let Err(err) = player.handle_message(msg) {
                eprintln!("Error handling player action: {}", err);
            }
        }
        _ = async {
                loop {
                    let _ = player.handle_idle();
                    sleep(Duration::from_millis(200)).await;
                }
            } => {println!("player stopped");}
    }

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
