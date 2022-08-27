use tui::backend::Backend;
use tui::Frame;
use tui::layout::Rect;
use tui::text::Spans;
use tui::widgets::{Block, Borders, Paragraph, Wrap};
use crate::app::App;
use crate::ui::Draw;

pub(crate) struct HelpPanel;

impl Draw for HelpPanel {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, app: &App, area: Rect) {
        let block = Block::default().title("Help:").borders(Borders::ALL);
        let paragraph = Paragraph::new(vec![Spans::from("Help:::")]).block(block).wrap(Wrap { trim: true });
        f.render_widget(paragraph, area);
    }
}