use termion::color;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("cannot highlight content")]
pub struct HighlightError;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Type {
    None,
    Number,
    Match,
    String,
    _Boolean,
    Char,
    Comment,
    MultilineComment,
    PrimaryKeywords,
    SecondaryKeywords,
}

impl Type {
    pub fn to_color(self) -> impl color::Color {
        match self {
            Type::Number => color::Rgb(220, 163, 163),
            Type::Match => color::Rgb(30, 139, 210),
            Type::String => color::Rgb(211, 54, 130),
            Type::_Boolean => color::Rgb(0, 0, 139),
            Type::Char => color::Rgb(108, 113, 196),
            Type::Comment | Type::MultilineComment => color::Rgb(133, 153, 0),
            Type::PrimaryKeywords => color::Rgb(181, 137, 0),
            Type::SecondaryKeywords => color::Rgb(42, 161, 152),
            Type::None => color::Rgb(255, 255, 255),
        }
    }
}
