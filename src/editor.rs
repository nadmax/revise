use std::io::{stdin, stdout, Write, Error};
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;
use termion::{clear, cursor};

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

            if let Err(error) = self.refresh_screen() {
                die(&error);
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

    fn refresh_screen(&self) -> Result<(), Error> {
        print!("{}{}", clear::All, cursor::Goto(1, 1));

        if self.should_quit {
            println!("Goodbye!\r");
        }

        stdout().flush()
    }
}

fn die(err: &Error) {
    print!("{}", clear::All);
    panic!("{}", err)
}

fn read_key() -> Result<Key, Error> {
    loop {
        if let Some(key) = stdin().lock().keys().next() {
            return key;
        }
    }
}