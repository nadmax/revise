#![warn(clippy::all)]
mod editor;
mod terminal;
mod error;
mod document;
mod row;
mod highlight;
mod filetype;

use editor::Editor;
pub use terminal::Terminal;
pub use editor::Position;
pub use editor::SearchDirection;
pub use error::content_error::CopyError;
pub use document::Document;
pub use row::Row;
pub use filetype::FileType;
pub use filetype::HighlightOptions;

fn main() {
    Editor::new().run();
}