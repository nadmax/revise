use crate::Position;

use crossterm::cursor;
use crossterm::event::{self, Event};
use crossterm::execute;
use crossterm::style::Color;
use crossterm::style::{ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal as CrossTerminal;
use std::io::{stdout, Error, Stdout, Write};
use std::time::Duration;

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    _stdout: Stdout,
}

impl Terminal {
    /// # Errors
    ///
    /// Will return `Error` if it fails to get terminal size  
    /// or if it fails to switch to raw mode
    pub fn new() -> Result<Self, Error> {
        CrossTerminal::enable_raw_mode()?;
        let size = CrossTerminal::size();

        match size {
            Ok(s) => Ok(Self {
                size: Size {
                    width: s.0,
                    height: s.1,
                },
                _stdout: stdout(),
            }),
            Err(err) => Err(err),
        }
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        let _ = execute!(
            stdout(),
            CrossTerminal::Clear(CrossTerminal::ClearType::All)
        );
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn cursor_position(position: &Position) {
        let Position { mut x, y } = position;

        x = x.saturating_add(1);

        let x = x as u16;
        let y = *y as u16;

        let _ = execute!(stdout(), cursor::MoveTo(x, y));
    }

    /// # Errors
    ///
    /// Will return an error if not
    /// all bytes could be written due to I/O errors
    /// or EOF being reached.
    pub fn flush() -> Result<(), Error> {
        stdout().flush()
    }

    /// # Errors
    ///
    /// Will return an error if it fails to read an event
    pub fn read_event() -> Result<Event, Error> {
        loop {
            if event::poll(Duration::from_millis(500))? {
                return event::read();
            }
        }
    }

    pub fn cursor_hide() {
        let _ = execute!(stdout(), cursor::Hide);
    }

    pub fn cursor_show() {
        let _ = execute!(stdout(), cursor::Show);
    }

    pub fn clear_current_line() {
        let _ = execute!(
            stdout(),
            CrossTerminal::Clear(CrossTerminal::ClearType::CurrentLine)
        );
    }

    pub fn set_bg_color(color: Color) {
        let _ = execute!(stdout(), SetBackgroundColor(color));
    }

    pub fn reset_color() {
        let _ = execute!(stdout(), ResetColor);
    }

    pub fn set_fg_color(color: Color) {
        let _ = execute!(stdout(), SetForegroundColor(color));
    }
}
