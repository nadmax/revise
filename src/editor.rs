use core::panic;
use std::io::{stdin, stdout, Error};
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn run(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        loop {
            if let Err(error) = self.process_keypress() {
                die(&error);
            }

            if self.should_quit {
                break;
            }
        }
    }

    pub fn default() -> Self {
        Self {should_quit: false}
    }

    fn process_keypress(&mut self) -> Result<(), Error> {
        let pressed_key = read_key()?;

        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }

        Ok(())
    }
}

fn die(err: &Error) {
    panic!("{}", err)
}

fn read_key() -> Result<Key, Error> {
    loop {
        if let Some(key) = stdin().lock().keys().next() {
            return key;
        }
    }
}