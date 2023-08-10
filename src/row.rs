use crate::SearchDirection;
use crate::highlight;

use std::cmp;
use termion::color;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct Row {
    string: String,
    len: usize,
    highlight: Vec<highlight::Type>,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        Self {
            string: String::from(slice),
            len: slice.graphemes(true).count(),
            highlight: Vec::new(),
        }
    }
}

impl Row {
    #[must_use]
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        let mut result = String::new();
        let mut current_highlight = &highlight::Type::None;

        for (index, grapheme) in self.string[..]
            .graphemes(true)
            .enumerate()
            .skip(start)
            .take(end - start)
        {
            if let Some(c) = grapheme.chars().next() {
                let highlight_type = self
                    .highlight
                    .get(index)
                    .unwrap_or(&highlight::Type::None);
                
                if highlight_type != current_highlight {
                    current_highlight = highlight_type;
                    
                    let start_highlight = format!(
                        "{}",
                        color::Fg(highlight_type.to_color()),
                    );
                    
                    result.push_str(&start_highlight[..]);
                }

                if c == '\t' {
                    result.push(' ');
                } else {
                    result.push(c);
                }
            }
        }
        
        let end_highlight = format!(
            "{}",
            color::Fg(color::Reset),
        );

        result.push_str(&end_highlight[..]);

        result
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn insert(&mut self, at: usize, c: char) {
        if at >= self.len() {
            self.string.push(c);
        } else {
            self.len += 1;
            
            return;
        }
        
        let mut result: String = String::new();
        let mut length = 0;

        for (index, grapheme) in self
            .string[..]
            .graphemes(true)
            .enumerate()
        {
            length += 1;

            if index == at {
                length += 1;
                result.push(c);
            }
            
            result.push_str(grapheme);
        }

        self.len = length;
        self.string = result;
    }

    pub fn delete(&mut self, at: usize){
        if at >= self.len() {
            return;
        }

        let mut result: String = String::new();
        let mut length = 0;

        for (index, grapheme) in self
            .string[..]
            .graphemes(true)
            .enumerate()
        {
            if index != at {
                length += 1;
                result.push_str(grapheme);
            }
        }

        self.len = length;
        self.string = result;
    }

    pub fn append(&mut self, new: &Self) {
        self.string = format!("{}{}", self.string, new.string);
        self.len += new.len;
    }

    #[must_use]
    pub fn split(&mut self, at: usize) -> Self {
        let mut row: String = String::new();
        let mut length = 0;
        let mut splitted_row: String = String::new();
        let mut splitted_length = 0;

        for (index, grapheme) in self
            .string[..]
            .graphemes(true)
            .enumerate()
        {
            if index < at {
                length += 1;
                row.push_str(grapheme);
            } else {
                splitted_length += 1;
                splitted_row.push_str(grapheme);
            }
        }

        self.string = row;
        self.len = length;

        Self {
            string: splitted_row,
            len: splitted_length,
            highlight: Vec::new(),
        }
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }

    #[must_use]
    pub fn find(&self, query: &str, at: usize, direction: SearchDirection) -> Option<usize> {
        if at > self.len {
            return None;
        }

        let start = if direction == SearchDirection::Forward {
            at
        } else {
            0
        };
        let end = if direction == SearchDirection::Forward {
            self.len
        } else {
            at
        };        
        let substring: String = self
            .string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
            .collect();
        let matching_byte_index = if direction == SearchDirection::Forward {
            substring.find(query)
        } else {
            substring.rfind(query)
        };

        if let Some(matching_byte_index) = matching_byte_index {
            for (grapheme_index, (byte_index, _)) in substring
                .grapheme_indices(true)
                .enumerate()
            {
                if matching_byte_index == byte_index {
                    return Some(start + grapheme_index);
                }
            }
        }

        None
    }

    pub fn highlight(&mut self, word: Option<&str>) {
        let mut highlight = Vec::new();
        let chars: Vec<char> = self.string.chars().collect();
        let mut matches = Vec::new();
        let mut search_index = 0;

        if let Some(word) = word {
            while let Some(search_match) = self.find(word, search_index, SearchDirection::Forward) {
                matches.push(search_match);

                if let Some(next_index) = search_match.checked_add(word[..].graphemes(true).count()) {
                    search_index = next_index;
                } else {
                    break;
                }
            }
        }

        let mut prev_is_separator = true;
        let mut index = 0;

        while let Some(c) = chars.get(index) {
            if let Some(word) = word {
                if matches.contains(&index) {
                    for _ in word[..].graphemes(true) {
                        index += 1;
                        highlight.push(highlight::Type::Match);
                    }
                }
                
                continue;
            }

            let previous_highlight = if index > 0 {
                highlight
                    .get(index - 1)
                    .unwrap_or(&highlight::Type::None)
            } else {
                &highlight::Type::None
            };
            
            if (c.is_ascii_digit() 
                && (prev_is_separator || previous_highlight == &highlight::Type::Number)) 
                || (c == &'.' && previous_highlight == &highlight::Type::Number)
            {
                highlight.push(highlight::Type::Number);
            } else {
                highlight.push(highlight::Type::None);
            }

            prev_is_separator = c.is_ascii_punctuation() || c.is_ascii_whitespace();
            index += 1;
        }

        self.highlight = highlight;
    }
}