use std::string::Drain;

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

    #[cfg(debug_assertions)]
    pub fn test() -> bool {
        let string = UnicodeString::new();
        if string.as_str() != "" || string.length() != 0 {
            println!("creating empty not working");
            return false;
        }

        let mut string = UnicodeString::from("test");
        if string.as_str() != "test" || string.length() != 4 {
            println!("creating from &str not working");
            return false;
        }

        string.insert(4, '!'); // test insertion at end
        string.insert(0, '_'); // test insertion at start
        string.insert(5, 'y'); // test insertion in middle
        string.push('_'); // test pushing
        if string.as_str() != "_testy!_" || string.length() != 8 {
            println!("insertion and push not working for '{}'", string.as_str());
            return false;
        }

        string.remove(0); // test removal at start
        string.remove(6); // test removal at end
        string.remove(4); // test removal in middle
        if string.as_str() != "test!" || string.length() != 5 {
            println!("removal not working for '{}'", string.as_str());
            return false;
        }

        while string.length() > 0 {
            string.remove(0); // test removing all characters
        }
        if string.as_str() != "" || string.length() != 0 {
            return false;
        }

        string.push('å'); // test pushing unicode
        string.push('ö'); // test pushing unicode
        string.insert(1, 'ä'); // test inserting unicode
        if string.as_str() != "åäö" || string.inner_indices != vec![0, 'å'.len_utf8(), "åä".len()] {
            return false;
        }

        string.remove(1); // test removing unicode in middle
        if string.as_str() != "åö" || string.inner_indices != vec![0, 'å'.len_utf8()] {
            return false;
        }

        let drained: String = string.drain(0, string.length()).collect(); // test draining all characters
        if drained.as_str() != "åö" || string.as_str() != "" || string.inner_indices != Vec::new() || string.length() != 0 {
            return false;
        } 

        string.push_str("asd"); // test pushing string literal
        if string.as_str() != "asd" || string.length() != 3 {
            return false;
        }

        let drained: String = string.drain(1, string.length()).collect(); // test draining some characters
        if drained.as_str() != "sd" || string.as_str() != "a" || string.inner_indices != vec![0] || string.length() != 1 {
            return false;
        }
        
        return true;
    }
}
