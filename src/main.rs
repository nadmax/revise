#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;
mod document;
mod row;

use editor::Editor;
pub use terminal::Terminal;
pub use editor::Position;
pub use editor::SearchDirection;
pub use document::Document;
pub use row::Row;

fn main() {
    Editor::new().run();
}