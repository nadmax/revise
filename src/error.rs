use std::error::Error;
use std::fmt::{Display, Formatter, Result};

pub mod errors {
    use super::*;

    #[derive(Debug)]
    pub struct CopyError(pub String);

    impl Display for CopyError {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{}", self.0)
        }
    }

    impl Error for CopyError {}

    #[derive(Debug)]
    pub struct HighlightError;

    impl Display for HighlightError {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "cannot highlight content")
        }
    }

    impl Error for HighlightError {}

    #[derive(Debug)]
    pub struct RowInsertionError(pub usize, pub usize);

    impl Display for RowInsertionError {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "cannot insert at position x: {}, y: {}",
                &self.0, &self.1
            )
        }
    }

    impl Error for RowInsertionError {}

    #[derive(Debug)]
    pub struct RowDeletionError(pub usize, pub usize);

    impl Display for RowDeletionError {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "cannot delete at position x: {}, y: {}",
                &self.0, &self.1
            )
        }
    }

    impl Error for RowDeletionError {}
}
