use std::env;
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;

use tokio::sync::mpsc;

use crate::player::Player;

use self::configuration::{Commands, Config, ConfigMode};
use self::grpc::server::GrpcServerMessage;

mod configuration;
mod file_explorer;
mod grpc;
mod player;

async fn handle_message(player: &mut Player, message: GrpcServerMessage) {
    match message {
        GrpcServerMessage::Play { resp } => {
            player.play();
            let _ = resp.send(Ok(()));
        }
        GrpcServerMessage::Pause { resp } => {
            player.pause();
            let _ = resp.send(Ok(()));
        }
        GrpcServerMessage::PlayPause { resp } => {
            player.play_pause();
            let _ = resp.send(Ok(()));
        }
        GrpcServerMessage::SkipSong { resp } => {
            let _ = match player.skip_song() {
                Ok(_) => resp.send(Ok(())),
                Err(err) => resp.send(Err(err.to_string())),
            };
        }
        GrpcServerMessage::Set { resp } => todo!(),
        GrpcServerMessage::GetFiles { path, resp } => {
            let files = player.get_files(&path).unwrap();
            let _ = resp.send(files);
        }
    }
}

async fn init_server(config: Config) -> Result<(), Box<dyn Error>> {
    let (tx, mut rx) = mpsc::channel::<grpc::server::GrpcServerMessage>(32);

    tokio::spawn(async move {
        let _ = grpc::GRPCServer::serve(config.address, tx).await;
    });

    let mut base_path = env::current_dir().expect("Error accesing the enviroment");
    let mut volume = 1.0;

    if let Commands::Start {
        base_path: config_path,
        volume: config_volume,
    } = config.command
    {
        base_path = config_path.to_owned();
        volume = config_volume;
    };

    let mut player = Player::new(base_path, volume).expect("Error creating player");

    println!("Listening for incomming messages...");

    // This macro will wait on multiple futures and will return when the first one resolves
    // TODO: create a break system for shutdown
    loop {
        tokio::select! {
            Some(msg) = rx.recv() => {
                handle_message(&mut player, msg).await;
            }
            _ = async {
                    loop {
                        let _ = player.handle_idle();
                        sleep(Duration::from_millis(200)).await;
                    }
                } => {}
            else => {
                println!("player stopped");
            }
        }
    }
}

async fn init_client(config: Config) -> Result<(), Box<dyn Error>> {
    let client = grpc::GRPCClient::new(config.address);

    match &config.command {
        Commands::Play => client.play().await?,
        Commands::Pause => client.pause().await?,
        Commands::PlayPause => client.play_pause().await?,
        Commands::SkipSong => client.skip_song().await?,
        Commands::Set => todo!(),
        Commands::GetFiles { path } => client.get_files(&path).await?,
        Commands::Ping => client.ping().await?,
        _ => {
            println!("This command doesn't apply to client mode")
        }
    }

    Ok(())
}

#[tokio::main()]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::new();

    match config.mode {
        ConfigMode::Server => init_server(config).await?,
        ConfigMode::Client => init_client(config).await?,
    };

    Ok(())
}
