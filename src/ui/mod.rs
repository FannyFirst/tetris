use tui::backend::Backend;
use tui::Frame;
use tui::layout::Rect;
use crate::app::App;

pub mod manager;
mod left_panel;
mod score_panel;
mod statistics_panel;

mod middle_panel;

mod right_panel;
mod next_panel;
mod help_panel;

trait Draw {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, app: &App,area: Rect);
}