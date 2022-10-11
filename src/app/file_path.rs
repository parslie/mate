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

    // TODO: actions

    // // //

    fn adjust_viewport(&mut self, area_width: usize) {
        if self.local_cursor_pos < self.viewport_offset {
            let diff = self.viewport_offset - self.local_cursor_pos;
            self.viewport_offset -= diff;
        } else if self.local_cursor_pos > area_width as usize - 1 {
            let diff = self.local_cursor_pos - (area_width as usize - 1);
            self.viewport_offset += diff;
        }
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