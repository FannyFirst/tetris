use std::error::Error;
use std::io::{stdout, Stdout};
use std::time::{Duration, Instant};
use crossterm::{event, execute};
use crossterm::event::{Event, KeyCode, MouseEventKind, DisableMouseCapture, EnableMouseCapture, poll};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use tui::backend::{Backend, CrosstermBackend};
use tui::Terminal;
use crate::ui;

pub struct App {
    shutdown: bool,
}

impl App {
    fn new() -> App {
        App {
            shutdown: false
        }
    }
    fn run<B: Backend>(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>> {
        let mut last_tick = Instant::now();
        let tick_rate = Duration::from_millis(250);
        loop {
            terminal.draw(|f| ui::home::draw(f, self))?;

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
            'q' => self.shutdown = true,
            _ => {}
        }
    }
    fn on_esc(&mut self) {
        self.shutdown = true
    }
    fn on_enter(&mut self) {}
    fn on_left(&mut self) {}
    fn on_right(&mut self) {}
    fn on_up(&mut self) {}
    fn on_down(&mut self) {}

    fn on_tick(&mut self) {}

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