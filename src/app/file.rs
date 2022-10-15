use tui::{layout::Rect, Frame, backend::Backend, text::{Spans, Text}, widgets::Paragraph};

use crate::app::unicode::UnicodeString;

pub struct File {
    pub path: UnicodeString,
    lines: Vec<UnicodeString>,
    local_cursor: (usize, usize),
    viewport_offset: (usize, usize),
}

impl File {
    pub fn new() -> Self {
        return Self {
            path: UnicodeString::new(),
            lines: vec![UnicodeString::new()],
            local_cursor: (0, 0),
            viewport_offset: (0, 0),
        };
    }

    // Miscellaneous

    fn get_line_mut(&mut self) -> &mut UnicodeString {
        return self.lines.get_mut(self.local_cursor.1).expect("should never index outside of file lines");
    }
    
    fn get_line(&self) -> &UnicodeString {
        return self.lines.get(self.local_cursor.1).expect("should never index outside of file lines");
    }
    
    fn clamped_file_cursor(&self) -> (usize, usize) {
        let line = self.get_line();
        if self.local_cursor.0 > line.length() {
            return (line.length(), self.local_cursor.1);
        } else {
            return self.local_cursor;
        }
    }

    pub fn to_string(&self) -> String {
        let mut output = String::new();
        for (i, line) in self.lines.iter().enumerate() {
            output.push_str(line.as_str());
            if i < self.lines.len() - 1 {
                output.push('\n');
            }
        }
        return output;
    }
    
    // Functionality
    
    pub fn write_character(&mut self, ch: char) {
        self.local_cursor = self.clamped_file_cursor();
    
        let char_idx = self.local_cursor.0;
        let line = self.get_line_mut();
        line.insert(char_idx, ch);
        self.local_cursor.0 += 1;
    }
    
    pub fn remove_character_before(&mut self) {
        self.local_cursor = self.clamped_file_cursor();
    
        if self.local_cursor.0 > 0 {
            let char_idx = self.local_cursor.0 - 1;
            let line = self.get_line_mut();
            line.remove(char_idx);
            self.local_cursor.0 -= 1;
        } else if self.local_cursor.1 > 0 {
            let curr_line = self.lines.remove(self.local_cursor.1);
            self.local_cursor.1 -= 1;
            let prev_line = self.get_line_mut();
            prev_line.push_str(curr_line.as_str());
            self.local_cursor.0 = prev_line.length() - curr_line.length();
        }
    }
    
    pub fn remove_character_after(&mut self) {
        self.local_cursor = self.clamped_file_cursor();
    
        if self.local_cursor.0 < self.get_line().length() {
            let char_idx = self.local_cursor.0;
            let line = self.get_line_mut();
            line.remove(char_idx);
        } else if self.local_cursor.1 < self.lines.len() - 1 {
            let next_line = self.lines.remove(self.local_cursor.1 + 1);
            let curr_line = self.get_line_mut();
            curr_line.push_str(next_line.as_str());
        }
    }
    
    pub fn break_line(&mut self) {
        self.local_cursor = self.clamped_file_cursor();
        let char_idx = self.local_cursor.0;
    
        let curr_line = self.get_line_mut();
        let curr_line_suffix: String = curr_line.drain(char_idx, curr_line.length()).collect();
        self.lines.insert(self.local_cursor.1 + 1, UnicodeString::from(curr_line_suffix.as_str()));
    
        self.local_cursor.0 = 0;
        self.local_cursor.1 += 1;
    }
    
    pub fn move_cursor_up(&mut self) {
        if self.local_cursor.1 > 0 {
            self.local_cursor.1 -= 1;
        }
    }
    
    pub fn move_cursor_down(&mut self) {
        if self.local_cursor.1 < self.lines.len() - 1 {
            self.local_cursor.1 += 1;
        }
    }
    
    pub fn move_cursor_left(&mut self) {
        self.local_cursor = self.clamped_file_cursor();
        if self.local_cursor.0 > 0 {
            self.local_cursor.0 -= 1;
        }
    }
    
    pub fn move_cursor_right(&mut self) {
        self.local_cursor = self.clamped_file_cursor();
        if self.local_cursor.0 < self.get_line().length() {
            self.local_cursor.0 += 1;
        }
    }
    
    // Rendering
    
    pub fn adjust_viewport(&mut self, rect: Rect) {
        if self.local_cursor.0 < self.viewport_offset.0 {
            let diff = self.viewport_offset.0 - self.local_cursor.0;
            self.viewport_offset.0 -= diff;
        } else if self.local_cursor.0 - self.viewport_offset.0 > (rect.width - 1) as usize {
            let diff = (self.local_cursor.0 - self.viewport_offset.0) - (rect.width - 1) as usize;
            self.viewport_offset.0 += diff;
        }
    
        if self.local_cursor.1 < self.viewport_offset.1 {
            let diff = self.viewport_offset.1 - self.local_cursor.1;
            self.viewport_offset.1 -= diff;
        } else if self.local_cursor.1 - self.viewport_offset.1 > (rect.height - 1) as usize {
            let diff = (self.local_cursor.1 - self.viewport_offset.1) - (rect.height - 1) as usize;
            self.viewport_offset.1 += diff;
        } 
    }
    
    pub fn global_cursor(&self, rect: Rect) -> (u16, u16) {
        let file_cursor = self.clamped_file_cursor();
        return (
            rect.x + (file_cursor.0 - self.viewport_offset.0) as u16,
            rect.y + (file_cursor.1 - self.viewport_offset.1) as u16,
        );
    }
    
    pub fn render<B: Backend>(&self, frame: &mut Frame<B>, rect: Rect) {
        let first_line_idx = self.viewport_offset.1;
        let last_line_idx = if self.viewport_offset.1 + rect.height as usize > self.lines.len() {
            self.lines.len()
        } else {
            self.viewport_offset.1 + rect.height as usize
        };
    
        let mut spans_vec = Vec::new();
        for line in &self.lines[first_line_idx..last_line_idx] {
            if line.length() >= self.viewport_offset.0 {
                spans_vec.push(Spans::from(&line[self.viewport_offset.0..]));
            } else {
                spans_vec.push(Spans::from(""));
            }
        }
    
        frame.render_widget(Paragraph::new(Text::from(spans_vec)), rect);
    }
}