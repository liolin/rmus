#[macro_use]
extern crate log;

pub mod model;
#[cfg(test)]
pub mod test_helpers;
pub mod ui;
pub mod util;

use crate::{
    ui::view::{self, View},
    util::Events,
};

use rodio::OutputStreamHandle;
use rodio::{Decoder, OutputStream, Sink};
use sqlx::SqlitePool;
use std::fs::File;
use std::io::BufReader;
use tui::Terminal;

pub struct App {
    pub view: view::View,
    pub pool: SqlitePool,
    pub player: Player,
    pub events: Events,
}

impl App {
    pub fn previous(&mut self) {
        match &mut self.view {
            View::Track(list) => {
                list.previous();
            }
            _ => {}
        }
    }

    pub fn next(&mut self) {
        match &mut self.view {
            View::Track(list) => {
                list.next();
            }
            _ => {}
        }
    }

    pub fn select(&mut self) {
        match &mut self.view {
            View::Track(list) => {
                if let Some(selected) = list.state.selected() {
                    let track = &list.items[selected];
                    self.player.play_new_track(&track.file_path);
                }
            }
            _ => {}
        }
    }

    pub fn unselect(&mut self) {
        match &mut self.view {
            View::Track(list) => {
                list.unselect();
            }
            _ => {}
        }
    }
}

pub struct Player {
    sink: Sink,
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
}

impl Player {
    pub fn new() -> Player {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        Player {
            sink,
            stream,
            stream_handle,
        }
    }

    pub fn play_new_track(&mut self, path: &str) {
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
}
