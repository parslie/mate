use tui::{Frame, backend::Backend, widgets::{Block, Borders, Paragraph}};

pub fn render_prompt<B: Backend>(frame: &mut Frame<B>, title: &str, desc: &str) {
    const MAX_WIDTH: u16 = 72;
    const HEIGHT: u16 = 3;

    let block = Block::default().borders(Borders::all()).title(title);
    let mut block_rect = frame.size();
    if block_rect.width > MAX_WIDTH {
        block_rect.x = (block_rect.width - MAX_WIDTH) / 2;
        block_rect.width = MAX_WIDTH;
    }
    block_rect.y = (block_rect.height - HEIGHT) / 2;
    block_rect.height = 3;

    frame.render_widget(Paragraph::new(desc), block.inner(block_rect));
    frame.render_widget(block, block_rect);
}
