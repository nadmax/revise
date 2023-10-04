#![warn(clippy::all)]
mod document;
mod revise;
mod filetype;
mod highlight;
mod keywords;
mod row;
mod terminal;

pub use document::Document;
use revise::Revise;
pub use revise::Position;
pub use revise::SearchDirection;
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
