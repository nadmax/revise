use std::error::Error;
use std::fmt::{Display, Formatter, Result};

pub mod content_error {
    use super::*;

    #[derive(Debug)]
    pub struct CopyError(pub String);
    
    impl Display for CopyError {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{}", self.0)
        }
    }
    
    impl Error for CopyError {}
}
