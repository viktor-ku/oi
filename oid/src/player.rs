use rodio::{Decoder, Device, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub struct Player {
    volume: f32,
    device: Device,
}

impl Player {
    pub fn new(volume: f32) -> Self {
        Self {
            volume,
            device: rodio::default_output_device().unwrap(),
        }
    }

    pub fn play(&self, path: &PathBuf) {
        let file = File::open(path).unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap();

        let sink = Sink::new(&self.device);
        sink.set_volume(self.volume);
        sink.append(source);
        sink.sleep_until_end();
    }
}
