use crate::config::Config;
use crate::ui::render_app;
use crate::TICK_RATE_MILLIS;
use anyhow::Result;
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::Stdout;
use std::time::Duration;

pub const TABS: &[&str] = &["webcam", "pc", "smartphone", "misc"];
pub const TAB_COUNT: usize = TABS.len();

#[derive(Default)]
pub struct App {
    pub config: Config,
    pub tab: usize,
    pub show_api_modal: bool,
}

impl App {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }
}

pub fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>, app: &mut App) -> Result<()> {
    let timeout = Duration::from_millis(TICK_RATE_MILLIS);
    loop {
        terminal.draw(|f| render_app(f, app))?;

        let event = event::read()?;

        if let Event::Key(key) = event {
            match key.code {
                // Exit app
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),

                // Switch Tabs
                KeyCode::Tab => app.tab = (app.tab + 1) % TAB_COUNT,
                KeyCode::BackTab => app.tab = (app.tab + TAB_COUNT - 1) % TAB_COUNT,
                KeyCode::Char('1') => app.tab = 0,
                KeyCode::Char('2') => app.tab = 1,
                KeyCode::Char('3') => app.tab = 2,
                KeyCode::Char('4') => app.tab = 3,

                KeyCode::Char('k') => app.show_api_modal = !app.show_api_modal,

                _ => {}
            }
        }

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if KeyCode::Char('q') == key.code {
                    break;
                }
            }
        }
    }
    Ok(())
}
