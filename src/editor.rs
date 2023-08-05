use crate::Terminal;

use std::io::Error;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::new().expect("Failed to initialize terminal"),
        }
    }

    pub fn run(&mut self) {
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

    fn process_keypress(&mut self) -> Result<(), Error> {
        let pressed_key = Terminal::read_key()?;

        if let Key::Ctrl('q') = pressed_key { self.should_quit = true}

        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(0, 0);
        
        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye!\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(0, 0);
        }

        Terminal::cursor_show();
        Terminal::flush()
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;

        for row in 0..height - 1 {
            Terminal::clear_current_line();
            
            if row == height / 3 {
                self.draw_welcome_message();        
            } else {
                println!("~\r");
            }
        }
    }
    
    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Editor -- version{VERSION}\r");
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = "".repeat(padding.saturating_sub(1));

        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);
        
        println!("{welcome_message}\r");
    }
}

fn die(err: &Error) {
    Terminal::clear_screen();

    panic!("{}", err)
}