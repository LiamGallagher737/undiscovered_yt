use crate::cleanup;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use colored::Colorize;
use crossterm::event::KeyEventKind::Press;
use crossterm::event::{read, Event, KeyCode, KeyModifiers};
use crossterm::terminal::{size, Clear, ClearType};
use crossterm::{cursor, ExecutableCommand};
use std::io::Stdout;
use yt_api::search::SearchResult;

pub fn results_interface(stdout: &mut Stdout, results: Vec<SearchResult>) -> Result<()> {
    let mut cursor = 0;
    loop {
        let (width, _) = size()?;
        for (n, result) in results.iter().enumerate() {
            let title = result
                .snippet
                .title
                .clone()
                .map(|t| t.chars().take(width as usize - 6).collect())
                .unwrap_or("Untitled".to_string())
                .bold();

            let info = format!(
                "Age: {} • Channel: {}",
                result
                    .snippet
                    .published_at
                    .map(video_age)
                    .unwrap_or("unknown".to_string()),
                result
                    .snippet
                    .channel_title
                    .clone()
                    .unwrap_or("anonymous".to_string())
            )
            .dimmed();

            println!();
            if cursor == n {
                println!("{} {}", "❯".bright_red(), title.bright_red());
                println!("{} {}", "❯".bright_red(), info.bright_red());
            } else {
                println!("  {title}");
                println!("  {info}");
            };
        }

        loop {
            let event = read()?;
            if let Event::Resize(_, _) = event {
                break;
            };
            let Event::Key(key) = event else {
                continue;
            };
            if key.kind != Press {
                continue;
            };
            let code = key.code;
            let last_cursor = cursor;
            match code {
                KeyCode::Esc => return Ok(()),
                KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => cleanup(stdout),
                KeyCode::Up => {
                    if cursor > 0 {
                        cursor -= 1
                    } else {
                        cursor = results.len() - 1
                    }
                }
                KeyCode::Down => {
                    if cursor < results.len() - 1 {
                        cursor += 1
                    } else {
                        cursor = 0
                    }
                }
                KeyCode::Char(c @ '1'..='9') => {
                    cursor = c.to_digit(10).context("Parsing error")? as usize - 1;
                    cursor = cursor.min(results.len() - 1);
                }
                KeyCode::Enter => {
                    open::that(format!(
                        "https://youtu.be/{}",
                        results[cursor].id.video_id.as_ref().unwrap()
                    ))
                    .context("Failed to open video in default browser")?;
                }
                _ => {}
            }
            if cursor != last_cursor {
                break;
            }
        }
        stdout.execute(cursor::MoveUp(results.len() as u16 * 3))?;
        stdout.execute(Clear(ClearType::FromCursorDown))?;
    }
}

fn video_age(publish_date: DateTime<Utc>) -> String {
    let diff = Utc::now() - publish_date;
    if diff.num_minutes() < 60 {
        format!("{} minutes", diff.num_minutes())
    } else if diff.num_hours() < 24 {
        format!("{} hours", diff.num_hours())
    } else {
        format!("{} days", diff.num_days())
    }
}
