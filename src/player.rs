use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use rodio::{OutputStream, Sink};

use crate::configuration;
use crate::file_explorer::walk_dir;

#[allow(dead_code)]
pub struct Player {
    queue: VecDeque<PathBuf>,
    sink: Sink,
    stream: OutputStream,
    base_dir: PathBuf,
}

impl std::ops::Deref for Player {
    type Target = Sink;

    fn deref(&self) -> &Self::Target {
        &self.sink
    }
}

impl Player {
    pub fn new(base_dir: PathBuf, volume: f32) -> Result<Self, Box<dyn Error>> {
        let queue = walk_dir(&base_dir)?;
        // stream needs to exist as long as sink to work
        let (stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;
        sink.set_volume(volume);

        Ok(Player {
            queue: VecDeque::from(queue),
            sink,
            stream,
            base_dir,
        })
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

    pub fn get_files(&mut self, path: &PathBuf) -> Result<Vec<PathBuf>, Box<dyn Error>> {
        // let mut base_path = env::current_dir().expect("Error accesing the enviroment");
        //
        // match path {
        //     Some(dir) => {
        //     }
        //     None => search_path = base_path.to_owned(),
        // }
        //
        // // PathBuf.join() can override the hole path, this ensure we're not accessing files outside
        // // base_dir
        // if !search_path.starts_with(base_path) {
        //     return Err("Tried to access file or directory outside of server `base_path` config.");
        // }

        let search_path = self
            .base_dir
            .join(path)
            .canonicalize()
            .expect("Couldn't canonicalizice the path");

        // PathBuf.join() can override the hole path, this ensure we're not accessing files outside base_dir
        if !search_path.starts_with(&self.base_dir) {
            panic!("Tried to access file or directory outside of server `base_path` config.")
        }

        Ok(walk_dir(&search_path)?)
    }

    pub fn play(&mut self) {
        self.sink.play();
    }

    pub fn pause(&mut self) {
        self.sink.pause();
    }

    pub fn play_pause(&self) {
        if self.sink.is_paused() {
            self.sink.play();
        } else {
            self.sink.pause();
        };
    }

    pub fn skip_song(&mut self) -> Result<(), Box<dyn Error>> {
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
