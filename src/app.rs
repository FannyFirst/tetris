use std::collections::HashMap;
use std::error::Error;
use std::io::{stdout, Stdout};
use std::time::{Duration, Instant};
use crossterm::{event, execute};
use crossterm::event::{Event, KeyCode, MouseEventKind, DisableMouseCapture, EnableMouseCapture, poll};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use tui::backend::{Backend, CrosstermBackend};
use tui::Terminal;
use crate::runtime::computing;
use crate::runtime::computing::Computer;
use crate::runtime::statistics::StatisticsInfo;
use crate::runtime::tetromino::{Point, Tetromino};
use crate::ui::manager::UIManager;

pub struct App {
    shutdown: bool,
    last_refresh: Option<Instant>,
    refresh_interval: Duration,
    pub score: Score,
    pub points: Vec<Vec<Point>>,
    pub active: Option<Tetromino>,
    pub next: Tetromino,
    pub statistics: HashMap<Tetromino, StatisticsInfo>,
}

pub struct Score {
    pub score: u32,
    pub top_score: u32,
}

impl App {
    fn new() -> App {
        App {
            shutdown: false,
            last_refresh: None,
            refresh_interval: Duration::from_millis(800),
            score: Score { score: 0, top_score: 0 },
            points: Vec::new(),
            active: Some(Tetromino::next_rng()),
            next: Tetromino::next_rng(),
            statistics: Default::default(),
        }
    }
    fn run<B: Backend>(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>> {
        let mut last_tick = Instant::now();
        let tick_rate = Duration::from_millis(250);
        loop {
            terminal.draw(|f| UIManager::draw(f, self))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if poll(timeout)? {
                match event::read() {
                    Ok(Event::Key(key)) =>
                        match key.code {
                            KeyCode::Char(c) => self.on_key(c),
                            KeyCode::Esc => {
                                self.on_esc();
                                break;
                            }
                            KeyCode::Enter => self.on_enter(),
                            KeyCode::Left => self.on_left(),
                            KeyCode::Right => self.on_right(),
                            KeyCode::Up => self.on_up(),
                            KeyCode::Down => self.on_down(),
                            _ => {}
                        }
                    Ok(Event::Mouse(mouse)) => {
                        match mouse.kind {
                            MouseEventKind::Down(_) => {}
                            MouseEventKind::Up(_) => {}
                            MouseEventKind::Drag(_) => {}
                            MouseEventKind::Moved => {}
                            MouseEventKind::ScrollDown => {}
                            MouseEventKind::ScrollUp => {}
                        }
                    }
                    _ => {}
                }
            }

            if last_tick.elapsed() >= tick_rate {
                self.on_tick();
                last_tick = Instant::now();
            }

            if self.is_shutdown() {
                break;
            }
        }
        Ok(())
    }

    fn on_key(&mut self, c: char) {
        match c {
            'q' | 'Q' => self.shutdown = true,
            _ => {}
        }
    }
    fn on_esc(&mut self) {
        self.shutdown = true
    }
    fn on_enter(&mut self) {}
    fn on_left(&mut self) {
        computing::move_left(self);
    }
    fn on_right(&mut self) {
        computing::move_right(self);
    }
    fn on_up(&mut self) {
        computing::rotation(self);
    }
    fn on_down(&mut self) {
        computing::step(self);
        self.last_refresh = Some(Instant::now());
    }

    fn on_tick(&mut self) {
        let mut need_step = true;
        if let Some(last_refresh) = self.last_refresh {
            if !last_refresh.elapsed().ge(&self.refresh_interval) {
                need_step = false;
            }
        }

        if need_step {
            computing::step(self);
            self.last_refresh = Some(Instant::now());
        }
    }

    fn is_shutdown(&self) -> bool {
        self.shutdown
    }
}


pub fn run() -> Result<(), Box<dyn Error>> {

    // setup terminal
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    // create app and run it
    App::new().run::<CrosstermBackend<Stdout>>(&mut terminal)?;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        DisableMouseCapture,
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    Ok(())
}