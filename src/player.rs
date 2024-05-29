use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use rodio::{OutputStream, Sink};

use crate::configuration::CONFIG;
use crate::file_explorer::walk_dir;

#[derive(Debug)]
pub enum PlayerAction {
    Play,
    SkipSong,
    Set,
}

pub struct Player {
    queue: VecDeque<PathBuf>,
    sink: Sink,
    stream: OutputStream,
}

impl std::ops::Deref for Player {
    type Target = Sink;

    fn deref(&self) -> &Self::Target {
        &self.sink
    }
}

impl Player {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let queue = walk_dir(&CONFIG.base_path)?;
        let (stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;
        sink.set_volume(CONFIG.volume);
        Ok(Player {
            queue: VecDeque::from(queue),
            sink,
            stream,
        })
    }

    pub fn handle_message(&mut self, message: PlayerAction) -> Result<(), Box<dyn Error>> {
        match message {
            PlayerAction::Play => self.play()?,
            PlayerAction::SkipSong => self.skip_song()?,
            PlayerAction::Set => unimplemented!(),
        }

        Ok(())
    }

    fn play(&mut self) -> Result<(), Box<dyn Error>> {
        while let Some(file_path) = self.queue.pop_front() {
            println!("Playing file: {}", file_path.display());
            self.enqueue_file(file_path)?;
            self.sink.sleep_until_end();
        }

        Ok(())
    }

    fn skip_song(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Skipping current song...:");
        self.sink.skip_one();

        Ok(())
    }

    fn enqueue_file(&self, file_path: PathBuf) -> Result<(), Box<dyn Error>> {
        let file = File::open(file_path)?;

        self.sink.append(rodio::Decoder::new(BufReader::new(file))?);
        Ok(())
    }

    fn _play_pause(&self) {
        if self.sink.is_paused() {
            self.sink.play();
        } else {
            self.sink.pause();
        };
    }

    fn set_playback_state(&self, is_paused: bool) {
        if is_paused {
            self.sink.pause();
        } else {
            self.sink.play();
        };
    }
}
