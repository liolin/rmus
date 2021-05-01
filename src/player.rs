use rodio::OutputStreamHandle;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

pub trait Player {
    fn play_new_track(&mut self, path: &str);
    fn toggle_pause(&mut self);
}

pub struct RodioPlayer {
    sink: Sink,
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
}

impl RodioPlayer {
    pub fn new() -> RodioPlayer {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        RodioPlayer {
            sink,
            stream,
            stream_handle,
        }
    }
}

impl Player for RodioPlayer {
    fn play_new_track(&mut self, path: &str) {
        if !self.sink.empty() {
            let (stream, stream_handle) = OutputStream::try_default().unwrap();
            self.stream = stream;
            self.stream_handle = stream_handle;
            self.sink = Sink::try_new(&self.stream_handle).unwrap();
        }

        let file = BufReader::new(File::open(path).unwrap());
        let source = Decoder::new(file).unwrap();

        self.sink.append(source);
        self.sink.play();
    }

    fn toggle_pause(&mut self) {
        if self.sink.is_paused() {
            self.sink.play();
        } else {
            self.sink.pause();
        }
    }
}
