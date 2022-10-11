use std::char::MAX;

use tui::{Frame, backend::Backend, widgets::{Block, Borders, Paragraph}, layout::Rect, text::Spans};

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

    // // //

    pub fn write_character(&mut self, ch: char) {
        self.path.insert(self.local_cursor_pos, ch);
        self.local_cursor_pos += 1;
    }

    pub fn remove_character_before(&mut self) {
        if self.local_cursor_pos > 0 {
            self.path.remove(self.local_cursor_pos - 1);
            self.local_cursor_pos -= 1;
        }
    }

    pub fn remove_character_after(&mut self) {
        if self.local_cursor_pos < self.path.length() {
            self.path.remove(self.local_cursor_pos);
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.local_cursor_pos > 0 {
            self.local_cursor_pos -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.local_cursor_pos < self.path.length() {
            self.local_cursor_pos += 1;
        }
    }

    // // //

    fn adjust_viewport(&mut self, area_width: usize) {
        if self.local_cursor_pos < self.viewport_offset {
            let diff = self.viewport_offset - self.local_cursor_pos;
            self.viewport_offset -= diff;
        } else if (self.local_cursor_pos - self.viewport_offset) > (area_width - 1) as usize {
            let diff = (self.local_cursor_pos - self.viewport_offset) - (area_width - 1) as usize;
            self.viewport_offset += diff;
        }
    }

    pub fn as_str(&self) -> &str {
        return self.path.as_str();
    }

    // // //

    pub fn render<B: Backend>(&mut self, frame: &mut Frame<B>) {
        const MAX_WIDTH: u16 = 72;
        const HEIGHT: u16 = 3;

        let block = Block::default().borders(Borders::all()).title(" Enter file path: ");
        let mut block_rect = frame.size();
        if block_rect.width > MAX_WIDTH {
            block_rect.x = (block_rect.width - MAX_WIDTH) / 2;
            block_rect.width = MAX_WIDTH;
        }
        block_rect.y = (block_rect.height - HEIGHT) / 2;
        block_rect.height = 3;
        let parag_rect = block.inner(block_rect);

        self.adjust_viewport(parag_rect.width as usize); // TODO: test if works
        let global_cursor_pos = parag_rect.x + (self.local_cursor_pos - self.viewport_offset) as u16;

        // Unlike OpenFile lines, should never be able to index outside of length
        let spans = Spans::from(&self.path[self.viewport_offset..]); 
        let parag = Paragraph::new(spans);

        frame.render_widget(parag, parag_rect);
        frame.render_widget(block, block_rect);
        frame.set_cursor(global_cursor_pos, parag_rect.y);
    }
}

impl PartialEq for FilePath {
    fn eq(&self, other: &Self) -> bool {
        return self.path == other.path;
    }
}