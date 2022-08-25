use crate::app::App;

use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Constraint, Direction, Layout};
use crate::ui::statistics;

pub fn draw<B: Backend>(f: &mut Frame<B>, _app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(20), Constraint::Percentage(60), Constraint::Percentage(20)])
        .direction(Direction::Horizontal)
        .split(f.size());
    statistics::draw(f, _app, chunks[0]);
    statistics::draw(f, _app, chunks[1]);
    statistics::draw(f, _app, chunks[2]);
}