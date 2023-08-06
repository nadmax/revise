use crate::Terminal;
use crate::Document;
use crate::Row;

use std::io::Error;
use std::env;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    document: Document,
}

impl Editor {
    pub fn new() -> Self {
        let args: Vec<String> = env::args().collect();
        let document = if args.len() > 1 {
            let filename = &args[1];
            Document::open(filename).unwrap_or_default()
        } else {
            Document::default()
        };

        Self {
            should_quit: false,
            terminal: Terminal::new().expect("Failed to initialize terminal"),
            cursor_position: Position::default(),
            document,
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

    pub fn draw_row(&self, row: &Row) {
        let end = self.terminal.size().width as usize;
        let row = row.render(0, end);

        println!("{row}\r");
    }

    fn process_keypress(&mut self) -> Result<(), Error> {
        let pressed_key = Terminal::read_key()?;

        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Up 
            | Key::Down 
            | Key::Left 
            | Key::Right 
            | Key::PageUp 
            | Key::PageDown
            | Key::End
            | Key::Home => self.move_cursor(pressed_key),
            _ => (),
        }

        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&self.cursor_position);
        
        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye!\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(&self.cursor_position);
        }

        Terminal::cursor_show();
        Terminal::flush()
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;

        for terminal_row in 0..height - 1 {
            Terminal::clear_current_line();
            
            if let Some(row) = self.document.row(terminal_row as usize) {
                self.draw_row(row);
            } else if self.document.is_empty() && terminal_row == height / 3 {
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

    fn move_cursor(&mut self, key: Key) {
        let Position {mut y, mut x} = self.cursor_position;
        let size = self.terminal.size();
        let height = size.height.saturating_sub(1) as usize;
        let width = size.width.saturating_sub(1) as usize;

        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1);  
                }
            }
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
                if x < width {
                    x = x.saturating_add(1);
                }
            }
            Key::PageUp => y = 0,
            Key::PageDown => y = height,
            Key::Home => x = 0,
            Key::End => x = width,
            _ => (),
        }

        self.cursor_position = Position {x, y}
    }
}

fn die(err: &Error) {
    Terminal::clear_screen();

    panic!("{}", err)
}