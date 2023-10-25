#![warn(clippy::all)]
mod application;
mod document;
mod filetype;
mod highlight;
mod keywords;
mod row;
mod terminal;

pub use application::Position;
use application::Revise;
pub use application::SearchDirection;
pub use document::Document;
pub use filetype::FileType;
pub use filetype::HighlightOptions;
pub use row::Row;
use std::error::Error;
pub use terminal::Terminal;

fn main() -> Result<(), Box<dyn Error>> {
    let mut revise = Revise::new()?;

    revise.run()?;

    Ok(())
}
