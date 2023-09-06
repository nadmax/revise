use crate::SearchDirection;
use crate::highlight;
use crate::HighlightOptions;

use std::cmp;
use termion::color;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct Row {
    pub is_highlighted: bool,
    string: String,
    len: usize,
    highlight: Vec<highlight::Type>,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        Self {
            is_highlighted: false,
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
                    .unwrap_or(current_highlight);

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
        self.is_highlighted = false;

        Self {
            is_highlighted: false,
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

    pub fn highlight(
        &mut self, 
        opts: &HighlightOptions, 
        word: &Option<String>,
        start_with_comment: bool,
    ) -> bool {
        let len = self.string.len();
        let chars: Vec<char> = self.string.chars().collect();

        if self.is_highlighted && word.is_none() {
            if let Some(hl_type) = self.highlight.last() {
                if *hl_type == highlight::Type::MultilineComment 
                    && len > 1 && self.string[len - 2..] == *"*/"
                {
                    return true;
                }
            }

            return false;
        }

        self.highlight = Vec::new();
        let mut index = 0;
        let mut in_ml_comment = start_with_comment;

        if in_ml_comment {
            let closing_index = if let Some(closing_index) = self.string.find("*/") {
                closing_index + 2
            } else {
                chars.len()
            };

            for _ in 0..closing_index {
                self.highlight.push(highlight::Type::MultilineComment);
            }

            index = closing_index;
        }

        while let Some(c) = chars.get(index) {
            if self.highlight_multiline_comment(&mut index, opts, *c, &chars) {
                in_ml_comment = true;

                continue;
            }

            in_ml_comment = false;

            if self.highlight_char(&mut index, opts, *c, &chars) 
                || self.highlight_comment(&mut index, opts, *c, &chars) 
                || self.highlight_primary_keywords(&mut index, opts, &chars) 
                || self.highlight_secondary_keywords(&mut index, opts, &chars) 
                || self.highlight_string(&mut index, opts, *c, &chars) 
                || self.highlight_number(&mut index, opts, *c, &chars)
            {
                continue;
            }

            self.highlight.push(highlight::Type::None);
            index += 1;
        }

        self.highlight_match(word);

        if in_ml_comment && &self.string[self.string.len().saturating_sub(2)..] != "*/" {
            return true;
        }

        self.is_highlighted = true;

        false
    }

    pub fn as_string(&self) -> &String {
        &self.string
    }

    fn highlight_match(&mut self, word: &Option<String>) {
        if let Some(word) = word {
            if word.is_empty() {
                return;
            }

            let mut index = 0;

            while let Some(search_match) = self.find(word, index, SearchDirection::Forward) {
                if let Some(next_index) = search_match.checked_add(
                    word[..].graphemes(true).count(),
                ) {
                    for i in index.saturating_add(search_match)..next_index {
                        self.highlight[i] = highlight::Type::Match;
                    }

                    index = next_index;
                } else {
                    break;
                }
            }
        }

    }

    fn highlight_str(
        &mut self,
        index: &mut usize,
        substring: &str,
        chars: &[char],
        hl_type: highlight::Type,
    ) -> bool {
        if substring.is_empty() {
            return false;
        }

        for (substring_index, c) in substring.chars().enumerate() {
            if let Some(next_char) = chars.get(index.saturating_add(substring_index)) {
                if *next_char != c {
                    return false;
                }
            } else {
                return false;
            }
        }

        for _ in 0..substring.len() {
            self.highlight.push(hl_type);
            *index += 1;
        }

        true
    }

    fn highlight_char(
        &mut self,
        index: &mut usize,
        opts: &HighlightOptions,
        c: char,
        chars: &[char],
    ) -> bool {
        if opts.char() && c == '\'' {
            if let Some(next_char) = chars.get(index.saturating_add(1)) {
                let closing_index = if *next_char == '\\' {
                    index.saturating_add(3)
                } else {
                    index.saturating_add(2)
                };

                if let Some(closing_char) = chars.get(closing_index) {
                    if *closing_char == '\'' {
                        for _ in 0..=closing_index.saturating_sub(*index) {
                            self.highlight.push(highlight::Type::Char);
                            *index += 1;
                        }

                        return true;
                    }
                }
            }
        }

        false
    }

    fn highlight_comment(
        &mut self,
        index: &mut usize,
        opts: &HighlightOptions,
        c: char,
        chars: &[char],
    ) -> bool {
        if opts.comments() && c == '/' && *index < chars.len() {
            if let Some(next_char) = chars.get(index.saturating_add(1)) {
                if *next_char == '/' {
                    for _ in *index..chars.len() {
                        self.highlight.push(highlight::Type::Comment);
                        *index += 1;
                    }

                    return true;
                }
            }
        }

        false
    }

    fn highlight_string(
        &mut self,
        index: &mut usize,
        opts: &HighlightOptions,
        c: char,
        chars: &[char],
    ) -> bool {
        if opts.strings() && c == '"' {
            loop {
                self.highlight.push(highlight::Type::String);
                *index += 1;

                if let Some(next_char) = chars.get(*index) {
                    if *next_char == '"' {
                        break;
                    }
                } else {
                    break;
                }
            }

            self.highlight.push(highlight::Type::String);
            *index += 1;

            return true;
        }

        false
    }

    fn highlight_number(
        &mut self,
        index: &mut usize,
        opts: &HighlightOptions,
        c: char,
        chars: &[char],
    ) -> bool {
        if opts.numbers() && c.is_ascii_digit() {
            if *index > 0 {
                let prev_char = chars[*index - 1];

                if !is_separator(prev_char) {
                    return false;
                }
            }
            loop {
                self.highlight.push(highlight::Type::Number);
                *index += 1;
                if let Some(next_char) = chars.get(*index) {
                    if *next_char != '.' && !next_char.is_ascii_digit() {
                        break;
                    }
                } else {
                    break;
                }
            }
            return true;
        }
        false
    }

    fn highlight_keywords(
        &mut self,
        index: &mut usize,
        chars: &[char],
        keywords: &[String],
        hl_type: highlight::Type,
    ) -> bool {
        if *index > 0 {
            let prev_char = chars[*index - 1];

            if !is_separator(prev_char) {
                return false;
            }
        }

        for word in keywords {
            if *index < chars.len().saturating_sub(word.len()) {
                let next_char = chars[*index + word.len()];

                if !is_separator(next_char) {
                    continue;
                }
            }

            if self.highlight_str(index, word, chars, hl_type) {
                return true;
            }
        }

        false
    }

    fn highlight_primary_keywords(
        &mut self,
        index: &mut usize,
        opts: &HighlightOptions,
        chars: &[char],
    ) -> bool {
        self.highlight_keywords(
            index,
            chars,
            opts.primary_keywords(),
            highlight::Type::PrimaryKeywords,
        )
    }

    fn highlight_secondary_keywords(
        &mut self,
        index: &mut usize,
        opts: &HighlightOptions,
        chars: &[char],
    ) -> bool {
        self.highlight_keywords(
            index,
            chars,
            opts.secondary_keywords(),
            highlight::Type::SecondaryKeywords,
        )
    }

    fn highlight_multiline_comment(
        &mut self,
        index: &mut usize,
        opts: &HighlightOptions,
        c: char,
        chars: &[char],
    ) -> bool {
        if opts.comments() && c == '/' && *index < chars.len() {
            if let Some(next_char) = chars.get(index.saturating_add(1)) {
                if *next_char == '*' {
                    let closing_index = 
                        if let Some(closing_index) = self.string[*index + 2..].find("*/") {
                            *index + closing_index + 4
                        } else {
                            chars.len()
                        };

                    for _ in *index..closing_index {
                        self.highlight.push(highlight::Type::MultilineComment);
                        *index += 1;
                    }

                    return true;
                }
            };
        }

        false
    }
}

fn is_separator(c: char) -> bool {
    c.is_ascii_punctuation() || c.is_ascii_whitespace()
}

#[cfg(test)]
mod row_tests {
    use super::*;

    #[test]
    fn test_find() {
        let row = Row::from("test123");

        assert_eq!(row.find("t", 0, SearchDirection::Forward), Some(0));
        assert_eq!(row.find("t", 2, SearchDirection::Forward), Some(3));
        assert_eq!(row.find("t", 5, SearchDirection::Forward), None);
    }

    #[test]
    fn test_highlight_match() {
        let mut row = Row::from("test123");

        row.highlight = vec![
            highlight::Type::Char,
            highlight::Type::Char,
            highlight::Type::Char,
            highlight::Type::Char,
            highlight::Type::Number,
            highlight::Type::Number,
            highlight::Type::Number,
        ];
        row.highlight_match(&Some("t".to_string()));
        assert_eq!(
            vec![
                highlight::Type::Match,
                highlight::Type::Char,
                highlight::Type::Char,
                highlight::Type::Char,
                highlight::Type::Number,
                highlight::Type::Number,
                highlight::Type::Number,
            ],
            row.highlight
        )
    }
}