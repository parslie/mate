use tui::{layout::Rect, backend::Backend, Frame, text::{Span, Spans}, widgets::Paragraph};

use super::unicode::UnicodeString;

pub struct Prompt {
    prompt: UnicodeString,
    answer: UnicodeString,
    local_cursor: usize,
    viewport_offset: usize,
}

impl Prompt {
    pub fn new(instruction: &str) -> Self {
        return Self {
            prompt: UnicodeString::from(instruction),
            answer: UnicodeString::new(),
            local_cursor: 0,
            viewport_offset: 0,
        };
    }

    pub fn set_answer(&mut self, new_answer: &UnicodeString) {
        self.answer = new_answer.clone();
        if self.local_cursor > new_answer.length() {
            self.local_cursor = new_answer.length();
        }
    }

    pub fn get_answer(&self) -> &UnicodeString {
        return &self.answer;
    }

    // Functionality

    pub fn write_character(&mut self, ch: char) {
        let char_idx = self.local_cursor;
        self.answer.insert(char_idx, ch);
        self.local_cursor += 1;
    }
    
    pub fn remove_character_before(&mut self) {
        if self.local_cursor > 0 {
            let char_idx = self.local_cursor - 1;
            self.answer.remove(char_idx);
            self.local_cursor -= 1;
        }
    }
    
    pub fn remove_character_after(&mut self) {
        if self.local_cursor < self.answer.length() {
            let char_idx = self.local_cursor;
            self.answer.remove(char_idx);
        }
    }
    
    pub fn move_cursor_left(&mut self) {
        if self.local_cursor > 0 {
            self.local_cursor -= 1;
        }
    }
    
    pub fn move_cursor_right(&mut self) {
        if self.local_cursor < self.answer.length() {
            self.local_cursor += 1;
        }
    }

    // Rendering

    pub fn adjust_viewport(&mut self, rect: Rect) {
        if self.local_cursor < self.viewport_offset {
            let diff = self.viewport_offset - self.local_cursor;
            self.viewport_offset -= diff;
        } else if self.local_cursor - self.viewport_offset > (rect.width - 1) as usize {
            let diff = (self.local_cursor - self.viewport_offset) - (rect.width - 1) as usize;
            self.viewport_offset += diff;
        }
    } 

    pub fn global_cursor(&self, rect: Rect) -> (u16, u16) {
        return (
            rect.x + (self.local_cursor - self.viewport_offset + self.prompt.length() + 2) as u16, // +2 to account for ": "
            rect.y
        );
    }

    pub fn render<B: Backend>(&self, frame: &mut Frame<B>, rect: Rect) {
        let mut span_vec = vec![Span::from(format!("{}: ", self.prompt.as_str()))];

        if self.answer.length() >= self.viewport_offset {
            span_vec.push(Span::from(&self.answer[self.viewport_offset..]));
        }

        frame.render_widget(Paragraph::new(Spans::from(span_vec)), rect);
    }
}