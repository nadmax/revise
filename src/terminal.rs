use std::io::{stdin, stdout, Stdout, Write, Error};
use termion::terminal_size;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{clear, cursor};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<Stdout>,
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
                height: size.1,
            },
            _stdout: stdout().into_raw_mode()?,
        })
    }

    #[must_use]
    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        print!("{}", clear::All);
    }

    pub fn cursor_position(x: u16, y: u16) {
        let x = x.saturating_add(1);
        let y = y.saturating_add(1);

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
}