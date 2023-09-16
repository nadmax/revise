#![warn(clippy::all)]
mod editor;
mod terminal;
mod error;
mod document;
mod row;
mod highlight;
mod filetype;

use editor::Editor;
use std::error::Error;
pub use terminal::Terminal;
pub use editor::Position;
pub use editor::SearchDirection;
pub use error::errors::{CopyError, RowDeletionError};
pub use document::Document;
pub use row::Row;
pub use filetype::FileType;
pub use filetype::HighlightOptions;

fn main() -> Result<(), Box<dyn Error>> {
    let mut editor = Editor::new()?;

    editor.run();

    Ok(())
}