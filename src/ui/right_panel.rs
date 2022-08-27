use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Constraint, Layout, Rect};
use crate::app::App;
use crate::ui::Draw;
use crate::ui::help_panel::HelpPanel;
use crate::ui::next_panel::NextPanel;

pub(crate) struct RightPanel;

impl Draw for RightPanel {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, app: &App, area: Rect) {
        let content = Layout::default()
            .constraints([Constraint::Length(6), Constraint::Min(3)])
            .split(area);
        NextPanel.draw(f, app, content[0]);
        HelpPanel.draw(f, app, content[1]);
    }
}