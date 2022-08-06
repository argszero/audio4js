use std::{fs::File, io::BufReader, time::Duration};

use rodio::{self, Source};

pub struct NodeRodioController {
    pub stream: (rodio::OutputStream, rodio::OutputStreamHandle),
    pub sink: rodio::Sink,
}
unsafe impl Send for NodeRodioController {}
unsafe impl Sync for NodeRodioController {}
impl NodeRodioController {
    pub fn new() -> Self {
        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&stream_handle).unwrap();
        NodeRodioController {
            sink,
            stream: (_stream, stream_handle),
        }
    }
    pub fn set_volume(&mut self, volume: f32) {
        self.sink.set_volume(volume);
    }
    pub fn sleep_until_end(&mut self) {
        self.sink.sleep_until_end();
    }
    pub fn play(&mut self, path: String, skip_secs: u64) -> Result<(), String> {
        match File::open(&path) {
            Ok(file) => match rodio::Decoder::new(BufReader::new(file)) {
                Ok(decorder) => {
                    self.sink
                        .append(decorder.skip_duration(Duration::from_secs(skip_secs)));
                }
                Err(e) => return Err(format!("{}", e)),
            },
            Err(e) => return Err(format!("{}", e)),
        }
        Ok(())
    }

    pub fn continue_play(&mut self) {
        self.sink.play();
    }
    pub fn pause(&mut self) {
        self.sink.pause();
    }
    pub fn stop(&mut self) {
        self.sink.stop();
        self.sink = rodio::Sink::try_new(&self.stream.1).unwrap();
    }
}
