use std::{fs::File, io::{self, Write}};

use tui::{backend::Backend, Frame, widgets::{Block, Borders, Paragraph}, text::{Spans, Text}, layout::Rect};

use super::{file_path::FilePath, unicode::UnicodeString};

pub struct OpenFile {
    pub path: FilePath,
    lines: Vec<UnicodeString>,
    local_cursor_pos: (usize, usize),
    viewport_offset: (usize, usize),
}

impl OpenFile {
    pub fn new() -> Self {
        return Self {
            path: FilePath::new(),
            lines: vec![UnicodeString::new()],
            local_cursor_pos: (0, 0),
            viewport_offset: (0, 0),
        };
    }

    // // //

    pub fn write_character(&mut self, ch: char) {
        self.local_cursor_pos.0 = self.clamped_local_cursor();
        let char_idx = self.local_cursor_pos.0;

        let line = self.get_line_mut();
        line.insert(char_idx, ch);
        self.local_cursor_pos.0 += 1;
    }

    pub fn remove_character_before(&mut self) {
        self.local_cursor_pos.0 = self.clamped_local_cursor();

        if self.local_cursor_pos.0 > 0 {
            let char_idx = self.local_cursor_pos.0 - 1;
            let line = self.get_line_mut();
            line.remove(char_idx);
            self.local_cursor_pos.0 -= 1;
        } else if self.local_cursor_pos.1 > 0 {
            let curr_line = self.lines.remove(self.local_cursor_pos.1);
            self.local_cursor_pos.1 -= 1;
            let prev_line = self.get_line_mut();
            prev_line.push_str(curr_line.as_str());
            self.local_cursor_pos.0 = prev_line.length() - curr_line.length();
        }
    }

    pub fn remove_character_after(&mut self) {
        self.local_cursor_pos.0 = self.clamped_local_cursor();

        if self.local_cursor_pos.0 < self.get_line().length() {
            let char_idx = self.local_cursor_pos.0;
            let line = self.get_line_mut();
            line.remove(char_idx);
        } else if self.local_cursor_pos.1 < self.lines.len() - 1 {
            let next_line = self.lines.remove(self.local_cursor_pos.1 + 1);
            let curr_line = self.get_line_mut();
            curr_line.push_str(next_line.as_str());
        }
    }

    pub fn break_line(&mut self) {
        self.local_cursor_pos.0 = self.clamped_local_cursor();
        let char_idx = self.local_cursor_pos.0;

        let curr_line = self.get_line_mut();
        let curr_line_suffix: String = curr_line.drain(char_idx, curr_line.length()).collect();
        self.lines.insert(self.local_cursor_pos.1 + 1, UnicodeString::from(curr_line_suffix.as_str()));

        self.local_cursor_pos.0 = 0;
        self.local_cursor_pos.1 += 1;
    }

    pub fn move_cursor_up(&mut self) {
        if self.local_cursor_pos.1 > 0 {
            self.local_cursor_pos.1 -= 1;
        }
    }

    pub fn move_cursor_down(&mut self) {
        if self.local_cursor_pos.1 < self.lines.len() - 1 {
            self.local_cursor_pos.1 += 1;
        }
    }

    pub fn move_cursor_left(&mut self) {
        self.local_cursor_pos.0 = self.clamped_local_cursor();
        if self.local_cursor_pos.0 > 0 {
            self.local_cursor_pos.0 -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        self.local_cursor_pos.0 = self.clamped_local_cursor();
        if self.local_cursor_pos.0 < self.get_line().length() {
            self.local_cursor_pos.0 += 1;
        }
    }

    // // //

    fn get_line_mut(&mut self) -> &mut UnicodeString {
        return self.lines.get_mut(self.local_cursor_pos.1).expect("should never index outside of line vector");
    }

    fn get_line(&self) -> &UnicodeString {
        return self.lines.get(self.local_cursor_pos.1).expect("should never index outside of line vector");
    }

    fn clamped_local_cursor(&self) -> usize {
        let line = self.get_line();
        if self.local_cursor_pos.0 > line.length() {
            return line.length();
        } else {
            return self.local_cursor_pos.0;
        }
    }

    fn adjust_viewport(&mut self, area_rect: Rect) {
        if self.local_cursor_pos.0 < self.viewport_offset.0 {
            let diff = self.viewport_offset.0 - self.local_cursor_pos.0;
            self.viewport_offset.0 -= diff;
        } else if self.local_cursor_pos.0 - self.viewport_offset.0 > (area_rect.width - 1) as usize {
            let diff = (self.local_cursor_pos.0 - self.viewport_offset.0) - (area_rect.width - 1) as usize;
            self.viewport_offset.0 += diff;
        }

        if self.local_cursor_pos.1 < self.viewport_offset.1 {
            let diff = self.viewport_offset.1 - self.local_cursor_pos.1;
            self.viewport_offset.1 -= diff;
        } else if self.local_cursor_pos.1 - self.viewport_offset.1 > (area_rect.height - 1) as usize {
            let diff = (self.local_cursor_pos.1 - self.viewport_offset.1) - (area_rect.height - 1) as usize;
            self.viewport_offset.1 += diff;
        } 
    }

    fn global_cursor_pos(&self, area_rect: Rect) -> (u16, u16) {
        return (
            area_rect.x + (self.clamped_local_cursor() - self.viewport_offset.0) as u16,
            area_rect.y + (self.local_cursor_pos.1 - self.viewport_offset.1) as u16
        );
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

    // // //

    pub fn render<B: Backend>(&mut self, frame: &mut Frame<B>) {
        let block = Block::default()
            .borders(Borders::all());

        let block_rect = frame.size();
        let parag_rect = block.inner(block_rect);

        let first_line_idx = self.viewport_offset.1;
        let last_line_idx = if parag_rect.height as usize + self.viewport_offset.1 > self.lines.len() {
            self.lines.len()
        } else {
            parag_rect.height as usize + self.viewport_offset.1
        };

        self.adjust_viewport(parag_rect); // TODO: test if works
        let global_cursor_pos = self.global_cursor_pos(parag_rect);

        let mut spans_vec = Vec::new();
        for line in &self.lines[first_line_idx..last_line_idx] {
            if line.length() >= self.viewport_offset.0 {
                spans_vec.push(Spans::from(&line[self.viewport_offset.0..]));
            } else {
                spans_vec.push(Spans::from(""));
            }
        }
        let parag = Paragraph::new(Text::from(spans_vec));

        frame.render_widget(parag, parag_rect);
        frame.render_widget(block, block_rect);
        frame.set_cursor(global_cursor_pos.0, global_cursor_pos.1);
    }
}