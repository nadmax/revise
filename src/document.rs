use crate::Row;
use crate::Position;

use std::fs;
use std::io::Error;
use std::cmp::Ordering;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub filename: Option<String>,
}

impl Document {
    /// # Errors
    /// 
    /// Will return `Error` if it fails read filename
    pub fn open(filename: &str) -> Result<Self, Error> {
        let contents = fs::read_to_string(filename)?;
        let mut rows = Vec::new();

        for value in contents.lines() {
            rows.push(Row::from(value));
        }

        Ok(Self {
            rows,
            filename: Some(filename.to_string()),
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
    }

    /// # Panics
    /// 
    /// Will panic if there is no row to delete
    pub fn delete(&mut self, at: &Position) {
        let len = self.len();

        if at.y >= len {
            return;
        }

        if at.x == self.rows.get_mut(at.y).unwrap().len() && at.y < len - 1 {
            let next_row = self.rows.remove(at.y + 1);
            let row = self.rows.get_mut(at.y).unwrap();

            row.append(&next_row);
        } else {
            let row = self.rows.get_mut(at.y).unwrap();

            row.delete(at.x);
        }

        let row = self.rows.get_mut(at.y).unwrap();

        row.delete(at.x);
    }

    fn insert_newline(&mut self, at: &Position) {
        let len = self.len();
        if at.y > len {
            return;
        }

        if at.y == len {
            self.rows.push(Row::default());

            return;
        }

        let new_row = self.rows.get_mut(at.y)
            .unwrap()
            .split(at.x);
        
        self.rows.insert(at.y + 1, new_row);
    }
}