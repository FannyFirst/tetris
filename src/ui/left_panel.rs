use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Constraint, Layout, Rect};
use tui::layout::Direction::Vertical;
use crate::app::App;
use crate::ui::Draw;
use crate::ui::score_panel::ScorePanel;
use crate::ui::statistics_panel::StatisticsPanel;

pub(crate) struct LeftPanel;

impl Draw for LeftPanel {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, app: &App, area: Rect) {
        let content = Layout::default().constraints([Constraint::Length(5), Constraint::Min(3)])
            .direction(Vertical)
            .split(area);
        ScorePanel.draw(f, app, content[0]);
        StatisticsPanel.draw(f, app, content[1]);
    }
}