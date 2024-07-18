use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use rodio::{OutputStream, Sink};

use crate::configuration::{self, CONFIG};
use crate::file_explorer::walk_dir;

#[allow(dead_code)]
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
        let queue = walk_dir(None)?;
        // stream needs to exist as long as sink to work
        let (stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;
        sink.set_volume(CONFIG.volume);

        Ok(Player {
            queue: VecDeque::from(queue),
            sink,
            stream,
        })
    }

    pub fn handle_message(
        &mut self,
        message: configuration::Commands,
    ) -> Result<(), Box<dyn Error>> {
        match message {
            configuration::Commands::Play => self.play(),
            configuration::Commands::Pause => self.pause(),
            configuration::Commands::PlayPause => self.play_pause(),
            configuration::Commands::SkipSong => self.skip_song()?,
            configuration::Commands::Set => todo!(),
            _ => {
                println!("This command doesn't apply to client mode")
            }
        };

        Ok(())
    }

    pub fn handle_idle(&mut self) -> Result<(), Box<dyn Error>> {
        if self.sink.is_paused() {
            return Ok(());
        }

        if self.queue.len() == 0 {
            return Ok(());
        }

        if self.sink.len() != 0 {
            return Ok(());
        }

        let file_path = self
            .queue
            .pop_front()
            .expect("There was an error with the queue");

        self.enqueue_file(file_path)?;

        Ok(())
    }

    fn play(&mut self) {
        self.sink.play();
    }

    fn pause(&mut self) {
        self.sink.pause();
    }

    fn play_pause(&self) {
        if self.sink.is_paused() {
            self.sink.play();
        } else {
            self.sink.pause();
        };
    }

    fn skip_song(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Skipping current song...:");
        let file_path = self.queue.pop_front().expect("foo");
        self.enqueue_file(file_path)?;
        self.sink.skip_one();

        Ok(())
    }

    fn enqueue_file(&self, file_path: PathBuf) -> Result<(), Box<dyn Error>> {
        println!("Playing file: {}", file_path.display());
        let file = File::open(file_path)?;

        self.sink.append(rodio::Decoder::new(BufReader::new(file))?);
        Ok(())
    }
}
