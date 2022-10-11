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

    // TODO: actions

    // // //

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
            area_rect.x + (self.local_cursor_pos.0 - self.viewport_offset.0) as u16,
            area_rect.y + (self.local_cursor_pos.1 - self.viewport_offset.1) as u16
        );
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