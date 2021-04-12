use std::io::{self, Stdout};

use anyhow::Result;
use termion::raw::{IntoRawMode, RawTerminal};
use tui::{backend::TermionBackend, Terminal};

pub mod view;
pub mod widget;

pub fn init_view() -> Result<Terminal<TermionBackend<RawTerminal<Stdout>>>> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    Ok(terminal)
}
