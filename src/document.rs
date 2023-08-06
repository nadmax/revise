use crate::Row;

use std::fs;
use std::io::Error;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
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
            rows
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
}