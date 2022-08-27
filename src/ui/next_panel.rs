use tui::backend::Backend;
use tui::Frame;
use tui::layout::Rect;
use tui::text::Spans;
use tui::widgets::{Block, Borders, Paragraph, Wrap};
use crate::app::App;
use crate::ui::Draw;

pub(crate) struct NextPanel;

impl Draw for NextPanel {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, app: &App, area: Rect) {
        let block = Block::default().title("Next:").borders(Borders::ALL);
        let paragraph = Paragraph::new(vec![Spans::from("Next")]).block(block).wrap(Wrap { trim: true });
        f.render_widget(paragraph, area);
    }
}