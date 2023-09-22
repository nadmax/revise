use crate::highlight::HighlightError;
use crate::row::RowError;
use crate::FileType;
use crate::Position;
use crate::Row;
use crate::SearchDirection;

use std::error::Error;
use std::fs::{read_to_string, File};
use std::io::{Error as IOError, Write};

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub filename: Option<String>,
    changed: bool,
    file_type: FileType,
}

impl Document {
    /// # Errors
    ///
    /// Will return `Error` if it fails read filename
    pub fn open(filename: &str) -> Result<Self, IOError> {
        let contents = read_to_string(filename)?;
        let file_type = FileType::new().from(filename);
        let mut rows = Vec::new();

        for value in contents.lines() {
            rows.push(Row::from(value));
        }

        Ok(Self {
            rows,
            filename: Some(filename.to_owned()),
            changed: false,
            file_type,
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

    pub fn insert(&mut self, at: &Position, c: char) -> Result<(), Box<dyn Error>> {
        if at.y > self.rows.len() {
            return Ok(());
        }

        self.changed = true;

        if c == '\n' {
            match self.insert_newline(at) {
                Ok(_) => (),
                Err(err) => return Err(err),
            }
        } else if at.y == self.rows.len() {
            let mut row = Row::default();
            row.insert(0, c);
            self.rows.push(row);
        } else {
            let row = self.rows.get_mut(at.y);

            match row {
                Some(r) => r.insert(at.x, c),
                None => return Err(Box::new(RowError::InsertionError(at.x, at.y))),
            }
        }

        self.unhighlight_rows(at.y);

        Ok(())
    }

    /// # Errors
    ///
    /// Will return `Error` if it fails to get the row to delete
    pub fn delete(&mut self, at: &Position) -> Result<(), Box<dyn Error>> {
        let len = self.len();

        if at.y >= len {
            return Ok(());
        }

        self.changed = true;
        let row = self.rows.get_mut(at.y);

        match row {
            Some(r) => {
                if at.x == r.len() && at.y < len - 1 {
                    let next_row = self.rows.remove(at.y + 1);
                    let row = self.rows.get_mut(at.y);

                    match row {
                        Some(r) => r.append(&next_row),
                        None => return Err(Box::new(RowError::DeletionError(at.x, at.y))),
                    }
                } else {
                    let row = self.rows.get_mut(at.y);

                    match row {
                        Some(r) => r.delete(at.x),
                        None => return Err(Box::new(RowError::DeletionError(at.x, at.y))),
                    }
                }

                self.unhighlight_rows(at.y);

                Ok(())
            }
            None => Err(Box::new(RowError::DeletionError(at.x, at.y))),
        }
    }

    /// # Errors
    ///
    /// Will return `Error` if it fails to create a file to save
    pub fn save(&mut self) -> Result<(), IOError> {
        if let Some(filename) = &self.filename {
            let mut file = File::create(filename)?;
            self.file_type = FileType::new().from(filename);

            for row in &mut self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }

            self.changed = false;
        }

        Ok(())
    }

    #[must_use]
    pub fn is_changed(&self) -> bool {
        self.changed
    }

    #[must_use]
    pub fn find(&self, query: &str, at: &Position, direction: SearchDirection) -> Option<Position> {
        if at.y >= self.rows.len() {
            return None;
        }

        let mut position = Position { x: at.x, y: at.y };
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
                    position.y = position.y.saturating_sub(1);
                    match self.rows.get(position.y) {
                        Some(row) => position.x = row.len(),
                        None => return None,
                    }
                }
            } else {
                return None;
            }
        }

        None
    }

    pub fn highlight(
        &mut self,
        word: &Option<String>,
        until: Option<usize>,
    ) -> Result<(), Box<dyn Error>> {
        let mut start_with_comment = false;
        let len = self.rows.len();
        let until = if let Some(until) = until {
            if until.saturating_add(1) < len {
                until.saturating_add(1)
            } else {
                len
            }
        } else {
            len
        };
        let rows = self.rows.get_mut(..until);

        match rows {
            Some(r) => {
                for row in r {
                    start_with_comment =
                        row.highlight(self.file_type.highlight_options(), word, start_with_comment);
                }
            }
            None => return Err(Box::new(HighlightError)),
        }

        Ok(())
    }

    pub fn file_type(&self) -> String {
        self.file_type.name()
    }

    fn insert_newline(&mut self, at: &Position) -> Result<(), Box<dyn Error>> {
        if at.y > self.rows.len() {
            return Ok(());
        }

        if at.y == self.rows.len() {
            self.rows.push(Row::default());
            return Ok(());
        }

        let current_row = self.rows.get_mut(at.y);

        match current_row {
            Some(row) => {
                let new_row = row.split(at.x);
                self.rows.insert(at.y + 1, new_row);

                Ok(())
            }
            None => return Err(Box::new(RowError::InsertionError(at.x, at.y))),
        }
    }

    fn unhighlight_rows(&mut self, start: usize) {
        let start = start.saturating_sub(1);

        for row in self.rows.iter_mut().skip(start) {
            row.is_highlighted = false;
        }
    }
}

#[cfg(test)]
mod document_tests {
    use super::*;

    #[test]
    fn test_save() {
        let mut new_doc = Document {
            filename: Some("test.txt".to_owned()),
            ..Document::default()
        };
        let save_res = new_doc.save();

        assert_eq!(save_res.ok(), Some(()),);
    }
}
