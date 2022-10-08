use tui::{text::{Text, Spans}, layout::Rect};

use super::unicode::UnicodeString;

pub struct OpenFile {
    name: Option<UnicodeString>,
    lines: Vec<UnicodeString>,
    local_cursor_pos: (usize, usize),
    viewport_offset: (usize, usize),
}

// TODO: when changing local_cursor_pos, make sure viewport_offset is <= to it
//       this needs to be done in write_character, remove_character, and break_line
//       preferably via a better clamped_local_cursor function
// TODO: clamp global_cursor_pos horizontally
impl OpenFile {
    pub fn new() -> Self {
        return Self {
            name: None,
            lines: vec![UnicodeString::new()],
            local_cursor_pos: (0, 0),
            viewport_offset: (0, 0),
        };
    }

    pub fn write_character(&mut self, area: Rect, ch: char) {
        self.local_cursor_pos = self.clamped_local_cursor();
        let curr_line = self.lines.get_mut(self.local_cursor_pos.1).expect("should never index outside of line vector");
        curr_line.insert(self.local_cursor_pos.0, ch);
        self.move_target_right(area);
    }

    pub fn remove_character(&mut self, area: Rect, before: bool) {
        self.local_cursor_pos = self.clamped_local_cursor();

        match before {
            true => {
                if self.local_cursor_pos.0 > 0 {
                    let curr_line = self.lines.get_mut(self.local_cursor_pos.1).expect("should never index outside of line vector");
                    curr_line.remove(self.local_cursor_pos.0 - 1);
                    self.move_target_left(area);
                } else if self.local_cursor_pos.1 > 0 {
                    let curr_line = self.lines.remove(self.local_cursor_pos.1);

                    self.move_target_up();
                    self.move_target_to_end_of_line(area);

                    let prev_line = self.lines.get_mut(self.local_cursor_pos.1).expect("should never index outside of line vector");
                    prev_line.push_str(curr_line.as_str());
                }
            }
            false => {
                let curr_line = self.lines.get_mut(self.local_cursor_pos.1).expect("should never index outside of line vector");

                if self.local_cursor_pos.0 < curr_line.length() {
                    curr_line.remove(self.local_cursor_pos.0);
                } else if self.local_cursor_pos.1 < self.lines.len() - 1 {
                    let next_line = self.lines.remove(self.local_cursor_pos.1 + 1);
                    let curr_line = self.lines.get_mut(self.local_cursor_pos.1).expect("should never index outside of line vector");
                    curr_line.push_str(next_line.as_str());
                }
            }
        }
    }

    pub fn break_line(&mut self, area: Rect) {
        self.local_cursor_pos = self.clamped_local_cursor();
        
        let curr_line = self.lines.get_mut(self.local_cursor_pos.1).expect("should never index outside of line vector");
        let curr_line_suffix: String = curr_line.drain(self.local_cursor_pos.0, curr_line.length()).collect();
        self.lines.insert(self.local_cursor_pos.1 + 1, UnicodeString::from(curr_line_suffix.as_str()));

        self.move_target_down(area);
        self.local_cursor_pos.0 = 0;
        self.viewport_offset.0 = 0;
    }

    pub fn to_text(&self, area: Rect) -> Text {
        let first_line_idx = self.viewport_offset.1;
        let last_line_idx = if area.height as usize + self.viewport_offset.1 > self.lines.len() {
            self.lines.len()
        } else {
            area.height as usize + self.viewport_offset.1
        };

        let mut lines_spans = Vec::new();
        for line in &self.lines[first_line_idx..last_line_idx] {
            let line_slice = if line.length() >= self.viewport_offset.0 {
                &line[self.viewport_offset.0..]
            } else {
                ""
            };
            lines_spans.push(Spans::from(line_slice));
        }
        return Text::from(lines_spans);
    }

    pub fn move_target_up(&mut self) {
        let global_cursor_pos = self.global_cursor();

        if self.local_cursor_pos.1 > 0 {
            self.local_cursor_pos.1 -= 1;
            if global_cursor_pos.1 == 0 {
                self.viewport_offset.1 -= 1;
            }
        }
    }

    pub fn move_target_down(&mut self, area: Rect) {
        let global_cursor_pos = self.global_cursor();

        if self.local_cursor_pos.1 < self.lines.len() - 1 {
            self.local_cursor_pos.1 += 1;
            if global_cursor_pos.1 >= area.height - 1 { // Should only be more than that if area is resized
                self.viewport_offset.1 += 1;
            }
        }
    }

    pub fn move_target_left(&mut self, area: Rect) {
        let global_cursor_pos = self.global_cursor();

        if self.local_cursor_pos.0 > 0 {
            self.local_cursor_pos.0 -= 1;
            if global_cursor_pos.0 == 0 {
                self.viewport_offset.0 -= 1;
            }
        }
    }

    pub fn move_target_right(&mut self, area: Rect) {
        let curr_line = self.lines.get(self.local_cursor_pos.1)
            .expect("should never index outside of lines vector");
        let global_cursor_pos = self.global_cursor();

        if self.local_cursor_pos.0 < curr_line.length() {
            self.local_cursor_pos.0 += 1;
            if global_cursor_pos.0 >= area.width - 1 { // Should only be more than that if area is resized
                self.viewport_offset.0 += 1;
            }
        }
    }

    pub fn move_target_to_end_of_line(&mut self, area: Rect) {
        let curr_line_len = self.lines.get(self.local_cursor_pos.1).expect("should never index outside of line vector").length();
        while self.local_cursor_pos.0 < curr_line_len {
            self.move_target_right(area); // TODO: implement faster approach
        }
    }

    pub fn move_target_to_start_of_line(&mut self, area: Rect) {
        while self.local_cursor_pos.0 > 0 {
            self.move_target_left(area); // TODO: implement faster approach
        }
    }

    // Getters

    pub fn global_cursor(&self) -> (u16, u16) {
        let mut global_cursor_pos: (u16, u16) = (0, 0);
        global_cursor_pos.0 = (self.local_cursor_pos.0 - self.viewport_offset.0) as u16;
        global_cursor_pos.1 = (self.local_cursor_pos.1 - self.viewport_offset.1) as u16;
        return global_cursor_pos;
    }

    pub fn clamped_global_cursor(&self) -> (u16, u16) {
        let curr_line_len = self.lines.get(self.local_cursor_pos.1).expect("should never index outside of line vector").length();
        if self.local_cursor_pos.0 > curr_line_len {
            let (global_x, global_y) = self.global_cursor();
            let diff = (self.local_cursor_pos.0 - curr_line_len) as u16;
            return (global_x - diff, global_y);
        } else {
            return self.global_cursor();
        }
    }

    fn clamped_local_cursor(&self) -> (usize, usize) {
        let curr_line = self.lines.get(self.local_cursor_pos.1).expect("should never index outside of line vector");

        if self.local_cursor_pos.0 > curr_line.length() {
            return (curr_line.length(), self.local_cursor_pos.1);
        } else {
            return self.local_cursor_pos;
        }
    }
}
