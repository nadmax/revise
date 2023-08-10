use crate::Row;
use crate::Position;
use crate::SearchDirection;

use std::fs::{File, read_to_string};
use std::io::{Error, Write};
use std::cmp::Ordering;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub filename: Option<String>,
    modified: bool,
}

impl Document {
    /// # Errors
    /// 
    /// Will return `Error` if it fails read filename
    pub fn open(filename: &str) -> Result<Self, Error> {
        let contents = read_to_string(filename)?;
        let mut rows = Vec::new();

        for value in contents.lines() {
            let mut row = Row::from(value);

            row.highlight(None);
            rows.push(row);
        }

        Ok(Self {
            rows,
            filename: Some(filename.to_string()),
            modified: false
        })
    }

    #[must_use]
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    /// # Panics
    /// 
    /// Will panic if there is no row to insert
    pub fn insert(&mut self, at: &Position, c: char) {
        let len = self.len();
        
        if c == '\n' {
            self.insert_newline(at);

            return;
        }

        match at.y.cmp(&len) {
            Ordering::Equal => {
                let mut row = Row::default();
        
                row.insert(0, c);
                self.rows.push(row);
            },
            Ordering::Less => {
                let row = self.rows.get_mut(at.y).unwrap();
                
                row.insert(at.x, c);
            },
            Ordering::Greater => (),
        }

        self.modified = true;
    }

    /// # Panics
    /// 
    /// Will panic if there is no row to delete
    pub fn delete(&mut self, at: &Position) {
        let len = self.len();

        if at.y >= len {
            return;
        }

        self.modified = true;

        if at.x == self.rows.get_mut(at.y).unwrap().len() && at.y < len - 1 {
            let next_row = self.rows.remove(at.y + 1);
            let row = self.rows.get_mut(at.y).unwrap();

            row.append(&next_row);
            row.highlight(None);
        } else {
            let row = self.rows.get_mut(at.y).unwrap();

            row.delete(at.x);
            row.highlight(None);
        }

        let row = self.rows.get_mut(at.y).unwrap();

        row.delete(at.x);
    }


    /// # Errors
    /// 
    /// Will return `Error` if it fails to create a file to save
    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(filename) = &self.filename {
            let mut file = File::create(filename)?;

            for row in &self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }

            self.modified = false;
        }

        Ok(())
    }

    #[must_use]
    pub fn is_modified(&self) -> bool {
        self.modified
    }

    #[must_use]
    pub fn find(&self, query: &str, at: &Position, direction: SearchDirection) -> Option<Position> {
        if at.y >= self.rows.len() {
            return None;
        }

        let mut position = Position {x: at.x, y: at.y };
        let start = if direction == SearchDirection::Forward {
            at.y
        } else {
            0
        };
        let end = if direction == SearchDirection::Forward {
            self.rows.len()
        } else {
            at.y.saturating_add(1)
        };

        for _ in start..end {
            if let Some(row) = self.rows.get(position.y) {
                if let Some(x) = row.find(query, position.x, direction) {
                    position.x = x;

                    return Some(position);
                }
                if direction == SearchDirection::Forward {
                    position.y = position.y.saturating_add(1);
                    position.x = 0;
                } else {
                    position.y = position.y.saturating_add(1);
                    position.x = self.rows[position.y].len();
                }
            } else {
                return None;
            }
        }

        None
    }

    pub fn highlight(&mut self, word: Option<&str>) {
        for row in &mut self.rows {
            row.highlight(word);
        }
    }

    fn insert_newline(&mut self, at: &Position) {
        let len = self.rows.len();
        
        match at.y.cmp(&len) {
            Ordering::Greater => (),
            Ordering::Equal => self.rows.push(Row::default()),
            Ordering::Less => {
                let current_row = &mut self.rows[at.y];
                let mut new_row = current_row.split(at.x);
        
                current_row.highlight(None);
                new_row.highlight(None);
                self.rows.insert(at.y + 1, new_row);
            },
        }
    }
}