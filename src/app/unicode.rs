use std::{string::Drain, ops::{Index, RangeFrom}};

#[derive(Clone)]
pub struct UnicodeString {
    inner_string: String,
    length: usize,
    inner_indices: Vec<usize>,
}

impl UnicodeString {
    pub fn new() -> Self {
        return Self {
            inner_string: String::new(),
            length: 0,
            inner_indices: Vec::new(),
        };
    }

    pub fn from(src: &str) -> Self {
        let mut new = Self {
            inner_string: String::from(src),
            length: 0,
            inner_indices: Vec::new(),
        };

        for (idx, _) in new.inner_string.char_indices() {
            new.length += 1;
            new.inner_indices.push(idx);
        }

        return new;
    }

    pub fn length(&self) -> usize {
        return self.length;
    }

    pub fn push(&mut self, ch: char) {
        self.inner_indices.push(self.inner_string.len());
        self.inner_string.push(ch);
        self.length += 1;
    }

    pub fn push_str(&mut self, s: &str) {
        for ch in s.chars() {
            self.push(ch);
        }
    }

    pub fn insert(&mut self, idx: usize, ch: char) {
        assert!(idx <= self.length);

        if idx == self.length {
            self.push(ch);
        } else {
            let inner_idx = self.inner_indices[idx];
            self.inner_string.insert(inner_idx, ch);

            self.inner_indices.insert(idx, inner_idx);
            for inner_idx in &mut self.inner_indices[idx+1..] {
                *inner_idx += ch.len_utf8();
            }

            self.length += 1;
        }
    }

    pub fn remove(&mut self, idx: usize) {
        assert!(idx < self.length);

        let inner_idx = self.inner_indices.remove(idx);
        let ch = self.inner_string.remove(inner_idx);

        for inner_idx in &mut self.inner_indices[idx..] {
            *inner_idx -= ch.len_utf8();
        }

        self.length -= 1;
    }

    pub fn drain(&mut self, start: usize, end: usize) -> Drain {
        assert!(start <= end);
        assert!(start <= self.length);
        assert!(end <= self.length);
        
        if start == end {
            return self.inner_string.drain(0..0);
        } else {
            let inner_start = self.inner_indices[start];
            let inner_end = if end == self.length {
                self.inner_string.len()
            } else {
                self.inner_indices[end]
            };

            let removed_bytes = inner_end - inner_start;
            self.inner_indices.drain(start..end);
            for inner_idx in &mut self.inner_indices[start..] {
                *inner_idx -= removed_bytes;
            }

            self.length -= end - start;
            return self.inner_string.drain(inner_start..inner_end);
        }
    }

    pub fn as_str(&self) -> &str {
        return self.inner_string.as_str();
    }
}

impl Index<RangeFrom<usize>> for UnicodeString {
    type Output = str;

    fn index(&self, idx: RangeFrom<usize>) -> &Self::Output {
        let inner_idx = if idx.start == self.length {
            self.inner_string.len()
        } else {
            self.inner_indices[idx.start]
        };
        return &self.inner_string[inner_idx..];
    }
}
