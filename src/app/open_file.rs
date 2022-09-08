use tui::text::{Text, Spans};

use super::unicode::UnicodeString;

pub struct OpenFile {
    name: Option<UnicodeString>,
    lines: Vec<UnicodeString>,
    target_char: usize,
    target_line: usize,
}

impl OpenFile {
    pub fn new() -> Self {
        return Self {
            name: None,
            lines: vec![UnicodeString::new()],
            target_char: 0,
            target_line: 0,
        };
    }

    pub fn clamped_target_char(&self) -> usize {
        let curr_line = self.lines.get(self.target_line).expect("should never index outside of line vector");
        return self.target_char.clamp(0, curr_line.length());
    }

    pub fn write_character(&mut self, ch: char) {
        self.target_char = self.clamped_target_char();

        let curr_line = self.lines.get_mut(self.target_line).expect("should never index outside of line vector");
        curr_line.insert(self.target_char, ch);
        self.target_char += 1;
    }

    pub fn remove_character(&mut self, before: bool) {
        self.target_char = self.clamped_target_char();

        match before {
            true => {
                if self.target_char > 0 {
                    let curr_line = self.lines.get_mut(self.target_line).expect("should never index outside of line vector");
                    curr_line.remove(self.target_char - 1);
                    self.target_char -= 1;
                } else if self.target_line > 0 {
                    let curr_line = self.lines.remove(self.target_line);
                    let prev_line = self.lines.get_mut(self.target_line - 1).expect("should never index outside of line vector");
                    self.target_char = prev_line.length();
                    self.target_line -= 1;
                    prev_line.push_str(curr_line.as_str());
                }
            }
            false => {
                let curr_line = self.lines.get_mut(self.target_line).expect("should never index outside of line vector");

                if self.target_char < curr_line.length() {
                    curr_line.remove(self.target_char);
                } else if self.target_line < self.lines.len() - 1 {
                    let next_line = self.lines.remove(self.target_line + 1);
                    let curr_line = self.lines.get_mut(self.target_line).expect("should never index outside of line vector");
                    curr_line.push_str(next_line.as_str());
                }
            }
        }
    }

    pub fn break_line(&mut self) {
        self.target_char = self.clamped_target_char();
        
        let curr_line = self.lines.get_mut(self.target_line).expect("should never index outside of line vector");
        let curr_line_suffix: String = curr_line.drain(self.target_char, curr_line.length()).collect();
        self.lines.insert(self.target_line + 1, UnicodeString::from(curr_line_suffix.as_str()));
        self.target_char = 0;
        self.target_line += 1;
    }

    pub fn to_text(&self) -> Text {
        let mut lines_spans = Vec::new();
        for line in &self.lines {
            lines_spans.push(Spans::from(line.as_str()));
        }
        return Text::from(lines_spans);
    }
}
