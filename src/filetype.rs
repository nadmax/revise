use crate::keywords;
use std::path::Path;

#[derive(Default)]
pub struct HighlightOptions {
    numbers: bool,
    strings: bool,
    char: bool,
    comments: bool,
    multiline_comments: bool,
    primary_keywords: Vec<String>,
    secondary_keywords: Vec<String>,
}

pub struct FileType {
    name: String,
    hl_opts: HighlightOptions,
}

impl HighlightOptions {
    pub fn numbers(&self) -> bool {
        self.numbers
    }

    pub fn strings(&self) -> bool {
        self.strings
    }

    pub fn char(&self) -> bool {
        self.char
    }

    pub fn comments(&self) -> bool {
        self.comments
    }

    pub fn primary_keywords(&self) -> &Vec<String> {
        &self.primary_keywords
    }

    pub fn secondary_keywords(&self) -> &Vec<String> {
        &self.secondary_keywords
    }

    pub fn multiline_comments(&self) -> bool {
        self.multiline_comments
    }
}

impl Default for FileType {
    fn default() -> Self {
        Self {
            name: String::from("No filetype"),
            hl_opts: HighlightOptions::default(),
        }
    }
}

impl FileType {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn from(filename: &str) -> Self {
        if Path::new(filename)
            .extension()
            .map_or(false, |ext| ext.eq_ignore_ascii_case("rs"))
        {
            return Self {
                name: String::from("Rust"),
                hl_opts: HighlightOptions {
                    numbers: true,
                    strings: true,
                    char: true,
                    comments: true,
                    multiline_comments: true,
                    primary_keywords: keywords::rust::primary_keywords(),
                    secondary_keywords: keywords::rust::secondary_keywords(),
                },
            };
        }

        Self::default()
    }

    pub fn highlight_options(&self) -> &HighlightOptions {
        &self.hl_opts
    }
}
