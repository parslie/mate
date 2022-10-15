use tui::{Frame, backend::Backend, layout::{Layout, Constraint, Direction}, widgets::{Block, Borders}};

use super::{Data, State};

pub fn render<B: Backend>(frame: &mut Frame<B>, data: &mut Data) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),
            Constraint::Length(2),
        ])
        .split(frame.size());

    let file_rect = layout[0];
    data.file.adjust_viewport(file_rect);
    data.file.render(frame, file_rect);

    let bar_block = Block::default().borders(Borders::TOP);
    let bar_content_rect = bar_block.inner(layout[1]);

    match data.state {
        State::Editing => {
            let cursor = data.file.global_cursor(file_rect);
            frame.set_cursor(cursor.0, cursor.1);
        },
        State::Saving => {
            data.save_prompt.adjust_viewport(bar_content_rect);
            data.save_prompt.render(frame, bar_content_rect);
            let cursor = data.save_prompt.global_cursor(bar_content_rect);
            frame.set_cursor(cursor.0, cursor.1);
        },
        State::Overwriting => {
            data.overwrite_prompt.adjust_viewport(bar_content_rect);
            data.overwrite_prompt.render(frame, bar_content_rect);
            let cursor = data.overwrite_prompt.global_cursor(bar_content_rect);
            frame.set_cursor(cursor.0, cursor.1);
        },
        _ => (),
    }

    frame.render_widget(bar_block, layout[1]);
}