use std::io;

use anyhow::Result;
use termion::raw::IntoRawMode;
use tui::{backend::TermionBackend, Terminal};

pub mod view;
pub mod widget;

type TermionTerminal =
    Terminal<tui::backend::TermionBackend<termion::raw::RawTerminal<std::io::Stdout>>>;

pub fn init_view() -> Result<TermionTerminal> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    Ok(terminal)
}
