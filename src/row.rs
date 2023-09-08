use crate::SearchDirection;
use crate::highlight;
use crate::HighlightOptions;

use std::cell::RefCell;
use bytes::BufMut;
use termion::color;

#[derive(Default, Clone)]
pub struct Row {
    pub is_highlighted: bool,
    buffer: RefCell<Vec<u8>>,
    len: usize,
    highlight: Vec<highlight::Type>,
}

impl From<u8> for Row {
    fn from(value: u8) -> Self {
        let data = vec![value];
        let buffer = vec![value];
        let len = String::from_utf8(data).unwrap().len();

        Self {
            is_highlighted: false,
            buffer: RefCell::new(buffer),
            len,
            highlight: Vec::new(),
        }
    }
}

impl Row {
    pub fn render(&self) -> Vec<u8> {
        let mut result = Vec::new();
        let mut current_highlight = &highlight::Type::None;

        for (index, byte) in self.buffer.borrow_mut().iter().enumerate()    {
            let highlight = self.highlight.get(index).unwrap_or(current_highlight);

            if highlight != current_highlight {
                current_highlight = highlight;

                result.put_slice(format!(
                    "{}",
                    color::Fg(current_highlight.to_color()),
                ).as_bytes());
            }

            if *byte == b'\t' {
                result.push(b' ');
            } else {
                result.push(*byte);
            }
        }

        let end_highlight = format!(
            "{}",
            color::Fg(color::Reset),
        );

        result.put_slice(end_highlight[..].as_bytes());

        result
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn get_buffer(&self) -> &RefCell<Vec<u8>> {
        &self.buffer
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn insert(&mut self, at: usize, slice: &[u8]) {
        if at >= self.len() {
            self.buffer.borrow_mut().put_slice(slice);
            self.len += 1;

            return;
        }

        let mut result = Vec::new();
        let mut length = 0;

        for (index, value) in slice.iter().enumerate() {
            length += 1;
            
            if index == at {
                length += 1;
                result.push(*value);
            }

            result.put_slice(slice)
        }

        self.len = length;
        self.buffer = RefCell::from(result);
    }

    pub fn delete(&mut self, at: usize) {
        if at >= self.len {
            return;

        }

        let mut result = Vec::new();
        let mut length = 0;
    
        for (index, value) in self.buffer.borrow_mut().iter().enumerate() {
            if index != at {
                length += 1;
                result.push(*value);
            }
        }

        self.len = length;
        self.buffer = RefCell::from(result);
    }

    pub fn append(&mut self, new: &mut Self) {
        self.buffer.borrow_mut().append(&mut new.buffer.borrow_mut());
        self.len += new.len;
    }

    pub fn split(&mut self, at: usize) -> Self {
        let mut row = Vec::new();
        let mut length = 0;
        let mut splitted_row = Vec::new();
        let mut splitted_length = 0;

        for (index, b) in self.buffer.borrow_mut().iter().enumerate() {
            if index < at {
                length += 1;
                row.push(*b);
            } else {
                splitted_length += 1;
                splitted_row.push(*b);
            }
        }

        self.buffer = RefCell::from(row);
        self.len = length;
        self.is_highlighted = false;

        Self {
            is_highlighted: false,
            buffer: RefCell::from(splitted_row),
            len: splitted_length,
            highlight: Vec::new(),
        }
    }

    pub fn find(&self, query: &[u8], at: usize, direction: SearchDirection) -> Option<usize> {
        if at > self.len {
            return None;
        }

        let start = match direction {
            SearchDirection::Forward => at,
            SearchDirection::Backward => 0,
        };
        let _end = match direction {
            SearchDirection::Forward => self.len,
            SearchDirection::Backward => at,
        };
        let borrowed_buf = self.buffer.borrow_mut();
        let subslice = borrowed_buf.as_slice();
        let matching_byte_index = match direction {
            SearchDirection::Forward => bfind(subslice, query),
            SearchDirection::Backward => rbfind(subslice, query),
        };

        if let Some(matching_byte_index) = matching_byte_index {  
            for (byte_index, _) in subslice.iter().enumerate() {
                if matching_byte_index == byte_index {
                    return Some(start + matching_byte_index);
                }
            }
        }

        None
    }

    pub fn highlight(
        &mut self,
        opts: &HighlightOptions,
        word: Option<&[u8]>,
        start_with_comment: bool,
    ) -> bool {
        if self.is_highlighted && word.is_none() {
            if let Some(hl_type) = self.highlight.last() {
                if *hl_type == highlight::Type::MultilineComment && self.len > 1 && bfind(&self.buffer.borrow_mut()[self.len - 2..], b"*/").is_some() {
                    return true;
                }
            }

            return false;
        }

        self.highlight = Vec::new();
        let mut index = 0;
        let mut in_ml_comment = start_with_comment;

        if in_ml_comment {
            let closing_index = if let Some(closing_index) = bfind(&self.buffer.borrow_mut().as_slice(), b"*/") {
                closing_index + 2
            } else {
                self.buffer.borrow_mut().len()
            };

            for _ in 0..closing_index {
                self.highlight.push(highlight::Type::MultilineComment);
            }

            index = closing_index;
        }

        let v = self.buffer.clone();

        while let Some(b) = v.borrow().get(index) {
            let cloned_buf = self.buffer.borrow().clone();

            if self.highlight_multiline_comment(&mut index, opts, *b, &cloned_buf.as_slice()) {
                in_ml_comment = true;

                continue;
            }

            in_ml_comment = false;

            if self.highlight_bytes(&mut index, opts, *b, &cloned_buf.as_slice()) 
                || self.highlight_comment(&mut index, opts, *b, &cloned_buf.as_slice()) 
                || self.highlight_primary_keywords(&mut index, opts, &cloned_buf.as_slice()) 
                || self.highlight_secondary_keywords(&mut index, opts, &cloned_buf.as_slice()) 
                || self.highlight_string(&mut index, opts, *b, &cloned_buf.as_slice()) 
                || self.highlight_number(&mut index, opts, *b, &cloned_buf.as_slice())
            {
                continue;
            }

            self.highlight.push(highlight::Type::None);
            index += 1;
        }

        self.highlight_match(word);

        if in_ml_comment && &self.buffer.borrow().as_slice()[self.buffer.borrow().len().saturating_sub(2)..] != b"*/" {
            return true;
        }

        self.is_highlighted = true;

        false
    }

    fn highlight_match(&mut self, word: Option<&[u8]>) {
        let slice = if let Some(word) = word {
            if word.is_empty() {
                return;
            } else {
                word
            }
        } else {
            return;
        };

        let mut index = 0;

        while let Some(search_match) = self.find(slice, index, SearchDirection::Forward) {
            if let Some(next_index) = search_match.checked_add(slice.len()) {
                for i in index.saturating_add(search_match)..next_index {
                    self.highlight[i] = highlight::Type::Match;
                }

                index = next_index;
            } else {
                break;
            }
        }
    }

    fn highlight_str(
        &mut self,
        index: &mut usize,
        slice: &[u8],
        hl_type: highlight::Type,
    ) -> bool {
        if slice.is_empty() {
            return false;
        }

        for (byte_index, value) in slice.iter().enumerate() {
            if let Some(next_byte) = slice.get(index.saturating_add(byte_index)) {
                if *next_byte != *value {
                    return false;
                }
            } else {
                return false;
            }
        }

        for _ in 0..slice.len() {
            self.highlight.push(hl_type);
            *index += 1;
        }

        true
    }


    fn highlight_bytes(
        &mut self,
        index: &mut usize,
        opts: &HighlightOptions,
        b: u8,
        slice: &[u8],
    ) -> bool {
        if opts.char() && b == b'\'' {
            if let Some(next_byte) = slice.get(index.saturating_add(1)) {
                let closing_index = if *next_byte == b'\\' {
                    index.saturating_add(3)
                } else {
                    index.saturating_add(2)
                };

                if let Some(closing_byte) = slice.get(closing_index) {
                    if *closing_byte == b'\'' {
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
        b: u8,
        slice: &[u8],
    ) -> bool {
        if opts.comments() && b == b'/' && *index < slice.len() {
            if let Some(next_byte) = slice.get(index.saturating_add(1)) {
                if *next_byte == b'/' {
                    for _ in *index..slice.len() {
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
        b: u8,
        slice: &[u8],
    ) -> bool {
        if opts.strings() && b == b'"' {
            loop {
                self.highlight.push(highlight::Type::String);
                *index += 1;

                if let Some(next_byte) = slice.get(*index) {
                    if *next_byte == b'"' {
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
        b: u8,
        slice: &[u8],
    ) -> bool {
        if opts.numbers() && b.is_ascii_digit() {
            if *index > 0 {
                let prev_byte = slice[*index - 1];

                if !is_separator(prev_byte) {
                    return false;
                }
            }

            loop {
                self.highlight.push(highlight::Type::Number);
                *index += 1;

                if let Some(next_byte) = slice.get(*index) {
                    if *next_byte != b'.' && !next_byte.is_ascii_digit() {
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
        slice: &[u8],
        keywords: &[String],
        hl_type: highlight::Type,
    ) -> bool {
        if *index > 0 {
            let prev_byte = slice[*index - 1];

            if !is_separator(prev_byte) {
                return false;
            }
        }

        for word in keywords {
            if *index < slice.len().saturating_sub(word.len()) {
                let next_byte = slice[*index + word.len()];

                if !is_separator(next_byte) {
                    continue;
                }
            }

            if self.highlight_str(index, slice, hl_type) {
                return true;
            }
        }

        false
    }

    fn highlight_primary_keywords(
        &mut self,
        index: &mut usize,
        opts: &HighlightOptions,
        slice: &[u8],
    ) -> bool {
        self.highlight_keywords(
            index,
            slice,
            opts.primary_keywords(),
            highlight::Type::PrimaryKeywords,
        )
    }

    fn highlight_secondary_keywords(
        &mut self,
        index: &mut usize,
        opts: &HighlightOptions,
        slice: &[u8],
    ) -> bool {
        self.highlight_keywords(
            index,
            slice,
            opts.secondary_keywords(),
            highlight::Type::SecondaryKeywords,
        )
    }

    fn highlight_multiline_comment(
        &mut self,
        index: &mut usize,
        opts: &HighlightOptions,
        b: u8,
        slice: &[u8],
    ) -> bool {
        if opts.multiline_comments() && b == b'/' && *index < slice.len() {
            if let Some(next_byte) = slice.get(index.saturating_add(1)) {
                if *next_byte == b'*' {
                    let closing_index = if let Some(closing_index) = bfind(&self.buffer.borrow_mut().as_slice()[*index + 2..], b"*/") {
                        *index + closing_index + 4
                    } else {
                        slice.len()
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

fn bfind(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

fn rbfind(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).rposition(|window| window == needle)
}

fn is_separator(b: u8) -> bool {
    b.is_ascii_punctuation() || b.is_ascii_whitespace()
}