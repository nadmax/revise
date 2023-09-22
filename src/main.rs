#![warn(clippy::all)]
mod document;
mod editor;
mod filetype;
mod highlight;
mod row;
mod terminal;
mod keywords;

pub use document::Document;
use editor::Editor;
pub use editor::Position;
pub use editor::SearchDirection;
pub use filetype::FileType;
pub use filetype::HighlightOptions;
pub use row::Row;
use std::error::Error;
pub use terminal::Terminal;

fn main() -> Result<(), Box<dyn Error>> {
    let mut editor = Editor::new()?;

    editor.run()?;

    Ok(())
}
