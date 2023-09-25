use crate::config::Config;
use crate::discover::{start_discovering, Discovery};
use crate::ui::render_app;
use crate::TICK_RATE_MILLIS;
use anyhow::Result;
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::Stdout;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;
use yt_api::search::SearchResult;

#[derive(Default)]
pub struct App {
    pub config: Config,
    pub tab: usize,
    pub results: Arc<Mutex<Vec<SearchResult>>>,
    pub discover_thread: Option<JoinHandle<()>>,
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

        if let Some(handle) = &app.discover_thread {
            if handle.is_finished() {
                app.discover_thread = None;
            }
        }

        if let Event::Key(key) = event {
            match key.code {
                // Exit app
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),

                KeyCode::Char('1') => {
                    app.tab = 0;
                    start_discovering(app, Discovery::Webcam);
                }
                KeyCode::Char('2') => {
                    app.tab = 1;
                    start_discovering(app, Discovery::Pc);
                }
                KeyCode::Char('3') => {
                    app.tab = 2;
                    start_discovering(app, Discovery::SmartPhone);
                }
                KeyCode::Char('4') => {
                    app.tab = 3;
                    start_discovering(app, Discovery::Misc);
                }

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
