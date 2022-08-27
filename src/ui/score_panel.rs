use std::ops::Add;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::Rect;
use tui::text::Spans;
use tui::widgets::{Block, Borders, Paragraph, Wrap};
use crate::app::App;
use crate::ui::Draw;

pub(crate) struct ScorePanel;

impl Draw for ScorePanel {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, app: &App, area: Rect) {
        let block = Block::default().title("Score:").borders(Borders::ALL);
        let text = vec![
            Spans::from(app.score.score.to_string()),
            Spans::from("top score: "),
            Spans::from(app.score.top_score.to_string()),
        ];
        let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
        f.render_widget(paragraph, area);
    }
}