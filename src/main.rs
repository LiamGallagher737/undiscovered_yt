use crate::config::Config;
use crate::discover::search;
use crate::questions::{discovery_type, extras_list, result_count};
use crate::results::results_interface;
use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::{cursor, ExecutableCommand};
use std::io;
use std::io::Stdout;

mod config;
mod discover;
mod questions;
mod results;

const BINARY_NAME: &str = "Undiscovered_YT";
const STARTUP_TITLE: &str = r"
  _   _           _ _                                     _  __   _______
 | | | |         | (_)                                   | | \ \ / /_   _|
 | | | |_ __   __| |_ ___  ___ _____   _____ _ __ ___  __| |  \ V /  | |
 | | | | '_ \ / _` | / __|/ __/ _ \ \ / / _ \ '__/ _ \/ _` |   \ /   | |
 | |_| | | | | (_| | \__ \ (_| (_) \ V /  __/ | |  __/ (_| |   | |   | |
  \___/|_| |_|\__,_|_|___/\___\___/ \_/ \___|_|  \___|\__,_|   \_/   \_/
";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Your YouTube API key (only needed once)
    #[arg(short = 'k', long)]
    api_key: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut config = Config::load().context("Failed to load config")?;

    if let Some(key) = args.api_key {
        config.api_key = key;
        config.save().context("Failed to save config")?;
        println!("Successfully saved API key! You may now run without the 'api_key' argument.");
        return Ok(());
    }

    let mut stdout = io::stdout();
    stdout.execute(EnableMouseCapture)?.execute(cursor::Hide)?;

    if config.title_text {
        // Ignore initial newline character
        println!("{}", &STARTUP_TITLE[1..].bright_red());
    }

    let selected_discovery = discovery_type(&mut stdout)?;
    let selected_extra = extras_list(&mut stdout)?;
    let result_count = result_count(&mut stdout)?;

    let results = search(
        selected_discovery,
        selected_extra,
        result_count,
        config.api_key,
    )?;

    println!();
    println!(
        "{} to move, {} to open, {} to exit",
        "<↑/↓>".bright_red(),
        "<enter>".bright_red(),
        "<esc>".bright_red()
    );

    results_interface(&mut stdout, results)?;

    cleanup(&mut stdout);
}

fn cleanup(stdout: &mut Stdout) -> ! {
    stdout
        .execute(DisableMouseCapture)
        .expect("Failed to disable mouse capture")
        .execute(cursor::Show)
        .expect("Failed to unhide cursor");
    println!();
    std::process::exit(0);
}
