use crate::app::App;

use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, BorderType};
use crate::ui::Draw;
use crate::ui::left_panel::LeftPanel;
use crate::ui::middle_panel::MiddlePanel;
use crate::ui::right_panel::RightPanel;

pub struct UIManager;

impl UIManager {
    pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {

        let title = Block::default()
            .borders(Borders::ALL)
            .title("Tetris")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);
        f.render_widget(title, f.size());


        let content = Layout::default()
            .constraints([Constraint::Length(30), Constraint::Percentage(40), Constraint::Length(30)])
            .direction(Direction::Horizontal)
            .margin(2)
            .split(f.size());

        LeftPanel.draw(f, app, content[0]);
        MiddlePanel.draw(f, app, content[1]);
        RightPanel.draw(f, app, content[2]);
    }
}