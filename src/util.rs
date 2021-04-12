use std::sync::mpsc;
use std::thread;

use anyhow::Result;
use termion::event::Key;
use termion::input::TermRead;

pub struct Events {
    rx: mpsc::Receiver<Key>,
}

impl Events {
    pub fn new() -> Events {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let stdin = std::io::stdin();
            for evt in stdin.keys() {
                if let Ok(key) = evt {
                    if let Err(err) = tx.send(key) {
                        eprintln!("{}", err);
                        return;
                    }
                }
            }
        });

        Events { rx }
    }

    pub fn next(&self) -> Result<Key> {
        let e = self.rx.recv()?;
        Ok(e)
    }
}
