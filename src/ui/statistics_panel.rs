use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Rect};
use tui::text::Spans;
use tui::widgets::{Block, Borders, Paragraph, Wrap};
use crate::app::App;
use crate::ui::Draw;

pub(crate) struct StatisticsPanel;

impl Draw for StatisticsPanel{
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, app: &App, area: Rect) {
        // let chunks = Layout::default().constraints([Constraint::Length(8)].to_vec())
        //     .split(area);
        let text = vec![
            Spans::from("Line1"),
            Spans::from("Line2"),
            Spans::from("Line3"),
            Spans::from("Line4"),
            Spans::from("Line5"),
            Spans::from("Line6"),
            Spans::from("Line7"),
            Spans::from("Line8"),
        ];

        let block = Block::default().borders(Borders::ALL)
            .title(Spans::from("Statistics"));
        let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
        f.render_widget(paragraph, area);
    }
}