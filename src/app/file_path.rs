use tui::text::{Spans, Span};

use super::unicode::UnicodeString;

#[derive(Clone)]
pub struct FilePath {
    path: UnicodeString,
    local_cursor_pos: usize,
    viewport_offset: usize,
}

impl FilePath {
    pub fn new() -> Self {
        return Self {
            path: UnicodeString::new(),
            local_cursor_pos: 0,
            viewport_offset: 0,
        };
    }

    pub fn write_character(&mut self, area_width: u16, ch: char) {
        self.path.insert(self.local_cursor_pos, ch);
        self.move_target_right(area_width);
    }

    pub fn remove_character(&mut self, before: bool) {
        if before {
            if self.local_cursor_pos > 0 && self.local_cursor_pos <= self.path.length() {
                self.path.remove(self.local_cursor_pos - 1);
                self.move_target_left();
            }
        } else if self.local_cursor_pos < self.path.length() {
            self.path.remove(self.local_cursor_pos);
        }
    }

    pub fn move_target_right(&mut self, area_width: u16) {
        let global_cursor_pos = self.global_cursor();

        if self.local_cursor_pos < self.path.length() {
            self.local_cursor_pos += 1;
            if global_cursor_pos >= area_width - 1 {
                self.viewport_offset += 1;
            }
        }
    }
    
    pub fn move_target_left(&mut self) {
        let global_cursor_pos = self.global_cursor();

        if self.local_cursor_pos > 0 {
            self.local_cursor_pos -= 1;
            if global_cursor_pos == 0 {
                self.viewport_offset -= 1;
            }
        }
    }

    // Getters

    pub fn global_cursor(&self) -> u16 {
        return (self.local_cursor_pos - self.viewport_offset) as u16;
    }

    pub fn to_spans(&self) -> Spans {
        if self.path.length() == 0 {
            return Spans::from("");
        } else {
            return Spans::from(&self.path[self.viewport_offset..]);
        }
    }
}