#![warn(clippy::all)]
mod document;
mod editor;
mod error;
mod filetype;
mod highlight;
mod row;
mod terminal;

pub use document::Document;
use editor::Editor;
pub use editor::Position;
pub use editor::SearchDirection;
pub use error::errors::*;
pub use filetype::FileType;
pub use filetype::HighlightOptions;
pub use row::Row;
use std::error::Error;
pub use terminal::Terminal;

fn main() -> Result<(), Box<dyn Error>> {
    let mut editor = Editor::new()?;

    editor.run();

    Ok(())
}
