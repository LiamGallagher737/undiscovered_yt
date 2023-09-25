use crate::app::{run_app, App};
use crate::config::{load_config, save_config};
use anyhow::{Context, Result};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::io::Stdout;

mod app;
mod config;
mod discover;
mod ui;

const BINARY_NAME: &str = "Undiscovered_YT";
const TICK_RATE_MILLIS: u64 = 250;

fn main() -> Result<()> {
    let mut terminal = setup_terminal().context("Terminal Setup Failed")?;

    // Create and run App
    let config = load_config().unwrap_or_default();
    let mut app = App::new(config);
    let res = run_app(&mut terminal, &mut app);

    // Save Config
    if let Err(e) = save_config(app.config) {
        println!("Error saving config!\n{e:#?}");
    }

    restore_terminal(&mut terminal)?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();
    enable_raw_mode().context("failed to enable raw mode")?;
    execute!(stdout, EnterAlternateScreen).context("unable to enter alternate screen")?;
    Terminal::new(CrosstermBackend::new(stdout)).context("creating terminal failed")
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode().context("failed to disable raw mode")?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .context("unable to switch to main screen")?;
    terminal.show_cursor().context("unable to show cursor")
}
