use crate::Position;

use std::io::{stdin, stdout, Stdout, Write, Error};
use termion::terminal_size;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{clear, cursor, color};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    stdout: RawTerminal<Stdout>,
}

impl Terminal {
    /// # Errors
    /// 
    /// Will return `Error` if it fails to get terminal size  
    /// or if it fails to switch to raw mode
    pub fn new() -> Result<Self, Error> {
        let size = terminal_size()?;

        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1.saturating_sub(2),
            },
            stdout: stdout().into_raw_mode()?,
        })
    }

    #[must_use]
    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        print!("{}", clear::All);
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn cursor_position(position: &Position) {
        let Position { mut x, mut y } = position;

        x = x.saturating_add(1);
        y = y.saturating_add(1);

        let x = x as u16;
        let y = y as u16;

        print!("{}", cursor::Goto(x, y));
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
    /// Will return an error if it fails to read key
    pub fn read_key() -> Result<Key, Error> {
        loop {
            if let Some(key) = stdin().lock().keys().next() {
                return key;
            }
        }
    }

    pub fn cursor_hide() {
        print!("{}", cursor::Hide);
    }

    pub fn cursor_show() {
        print!("{}", cursor::Show);
    }

    pub fn clear_current_line() {
        print!("{}", clear::CurrentLine);
    }

    pub fn set_bg_color(color: color::Rgb) {
        print!("{}", color::Bg(color));
    }

    pub fn reset_bg_color() {
        print!("{}", color::Bg(color::Reset));
    }

    pub fn set_fg_color(color: color::Rgb) {
        print!("{}", color::Fg(color));
    }

    pub fn reset_fg_color() {
        print!("{}", color::Fg(color::Reset));
    }

    pub fn write(&mut self, buf: &[u8]) {
        let _ = self.stdout.write(buf);
        let _ = self.stdout.write(b"\n");
    }

}