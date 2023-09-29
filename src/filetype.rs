use crate::keywords;
use std::error::Error as Err;
use std::path::Path;
use thiserror::Error;

#[derive(Default)]
pub struct HighlightOptions {
    numbers: bool,
    strings: bool,
    boolean: bool,
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

#[derive(Debug, Error)]
#[error("failed to parse extension from filename: {0}")]
struct ParseExtensionError(String);

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

    pub fn boolean(&self) -> bool {
        self.boolean
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
    pub fn new() -> Self {
        Self {
            name: String::new(),
            hl_opts: HighlightOptions::default(),
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn from(&self, filename: &str) -> Self {
        match self.parse_extension(filename) {
            Ok(ext) => match ext.as_str() {
                "rs" => return self.create_file_type("Rust"),
                "toml" => return self.create_file_type("Toml"),
                "lock" => return self.create_file_type("Lock"),
                "md" => return self.create_file_type("Markdown"),
                "yml" => return self.create_file_type("YAML"),
                _ => Self::default(),
            },
            Err(_) => return self.create_file_type(filename),
        }
    }

    pub fn highlight_options(&self) -> &HighlightOptions {
        &self.hl_opts
    }

    fn parse_extension(&self, filename: &str) -> Result<String, Box<dyn Err>> {
        let path = Path::new(filename);

        match path.extension() {
            Some(ext) => match ext.to_str() {
                Some(ext) => return Ok(ext.to_owned()),
                None => return Err(Box::new(ParseExtensionError(filename.to_owned()))),
            },
            None => return Err(Box::new(ParseExtensionError(filename.to_owned()))),
        }
    }

    fn create_file_type(&self, file_type: &str) -> Self {
        Self {
            name: String::from(file_type),
            hl_opts: HighlightOptions {
                numbers: true,
                strings: true,
                boolean: true,
                char: true,
                comments: true,
                multiline_comments: true,
                primary_keywords: keywords::rust::primary_keywords(),
                secondary_keywords: keywords::rust::secondary_keywords(),
            },
        }
    }
}
