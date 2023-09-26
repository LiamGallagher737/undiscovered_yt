use crate::cleanup;
use crate::discover::{Discovery, Extra};
use anyhow::Result;
use colored::Colorize;
use crossterm::event::KeyEventKind::Press;
use crossterm::event::{read, Event, KeyCode, KeyModifiers};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor, ExecutableCommand};
use std::io::{Stdout, Write};

pub fn discovery_type(stdout: &mut Stdout) -> Result<Discovery> {
    let question = "?".bright_green();
    println!(
        "{question} {}",
        "What discovery type would you like to use?".bold()
    );
    let mut selected_discovery = 0;
    'outer: loop {
        for (n, option) in Discovery::VARIANTS.iter().enumerate() {
            if n == selected_discovery {
                println!("{} {}", "❯".bright_red(), option.bright_red().bold());
            } else {
                println!("  {}", option);
            }
        }
        loop {
            let Event::Key(key) = read()? else {
                continue;
            };
            if key.kind != Press {
                continue;
            };
            let code = key.code;
            let old_selected = selected_discovery;
            match code {
                KeyCode::Esc => cleanup(stdout),
                KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => cleanup(stdout),
                KeyCode::Up => {
                    if selected_discovery > 0 {
                        selected_discovery -= 1
                    } else {
                        selected_discovery = Discovery::VARIANTS.len() - 1
                    }
                }
                KeyCode::Down => {
                    if selected_discovery < Discovery::VARIANTS.len() - 1 {
                        selected_discovery += 1
                    } else {
                        selected_discovery = 0
                    }
                }
                KeyCode::Char('1') => selected_discovery = 0,
                KeyCode::Char('2') => selected_discovery = 1,
                KeyCode::Char('3') => selected_discovery = 2,
                KeyCode::Char('4') => selected_discovery = 3,
                KeyCode::Enter => {
                    break 'outer;
                }
                _ => {}
            }
            if old_selected != selected_discovery {
                break;
            }
        }
        stdout.execute(cursor::MoveUp(Discovery::VARIANTS.len() as u16))?;
        stdout.execute(Clear(ClearType::FromCursorDown))?;
    }

    stdout.execute(cursor::MoveUp(Discovery::VARIANTS.len() as u16 + 1))?;
    stdout.execute(Clear(ClearType::FromCursorDown))?;

    println!(
        "{question} {} {}",
        "What discovery type would you like to use?".bold(),
        Discovery::VARIANTS[selected_discovery].dimmed(),
    );

    Ok(Discovery::from_index(selected_discovery))
}

pub fn extras_list(stdout: &mut Stdout) -> Result<Vec<Extra>> {
    let question = "?".bright_green();
    println!(
        "{question} {} ({} to select, {} to proceed)",
        "Which extras would you like to enable?".bold(),
        "<space>".bright_red(),
        "<enter>".bright_red(),
    );
    let mut extras_cursor = 0;
    let mut selected_extras = Vec::new();
    'outer: loop {
        for (n, variant) in Extra::VARIANTS.iter().enumerate() {
            let dot = if selected_extras.contains(&Extra::from_index(n)) {
                "◉"
            } else {
                "◯"
            };
            if n == extras_cursor {
                println!(
                    "{} {} {}",
                    "❯".bright_red(),
                    dot.bright_red(),
                    variant.bright_red().bold()
                );
            } else {
                println!("  {} {}", dot, variant);
            }
        }
        loop {
            let Event::Key(key) = read()? else {
                continue;
            };
            if key.kind != Press {
                continue;
            };
            let code = key.code;
            let old_selected = extras_cursor;
            match code {
                KeyCode::Esc => cleanup(stdout),
                KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => cleanup(stdout),
                KeyCode::Up => {
                    if extras_cursor > 0 {
                        extras_cursor -= 1
                    } else {
                        extras_cursor = Extra::VARIANTS.len() - 1
                    }
                }
                KeyCode::Down => {
                    if extras_cursor < Extra::VARIANTS.len() - 1 {
                        extras_cursor += 1
                    } else {
                        extras_cursor = 0
                    }
                }
                KeyCode::Char('1') => extras_cursor = 0,
                KeyCode::Char(' ') => {
                    let extra = Extra::from_index(extras_cursor);
                    if let Some(n) = selected_extras.iter().position(|v| *v == extra) {
                        selected_extras.remove(n);
                    } else {
                        selected_extras.push(extra);
                    };
                    break;
                }
                KeyCode::Enter => {
                    break 'outer;
                }
                _ => {}
            }
            if old_selected != extras_cursor {
                break;
            }
        }
        stdout.execute(cursor::MoveUp(Extra::VARIANTS.len() as u16))?;
        stdout.execute(Clear(ClearType::FromCursorDown))?;
    }

    stdout.execute(cursor::MoveUp(Extra::VARIANTS.len() as u16 + 1))?;
    stdout.execute(Clear(ClearType::FromCursorDown))?;

    println!(
        "{question} {} {}",
        "Which extras would you like to enable?".bold(),
        if selected_extras.is_empty() {
            "None".dimmed()
        } else {
            selected_extras
                .iter()
                .map(|e| Extra::VARIANTS[*e as usize])
                .collect::<Vec<&str>>()
                .join(", ")
                .dimmed()
        }
    );

    Ok(selected_extras)
}

pub fn result_count(stdout: &mut Stdout) -> Result<usize> {
    let question = "?".bright_green();
    println!(
        "{question} {}",
        "How many results would you like to search for?".bold()
    );

    stdout.execute(cursor::Show)?;

    let mut input = String::from("5");
    'outer: loop {
        print!("{} {input}", "❯❯".bright_red());
        stdout.flush()?;
        loop {
            let Event::Key(key) = read()? else {
                continue;
            };
            if key.kind != Press {
                continue;
            };
            let code = key.code;
            match code {
                KeyCode::Esc => cleanup(stdout),
                KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => cleanup(stdout),
                KeyCode::Char(c @ '0'..='9') => {
                    input.push(c);
                    break;
                }
                KeyCode::Backspace => {
                    if !input.is_empty() {
                        input.remove(input.len() - 1);
                    }
                    break;
                }
                KeyCode::Enter => {
                    break 'outer;
                }
                _ => {}
            }
        }
        stdout.execute(cursor::SavePosition)?;
        stdout.execute(Clear(ClearType::CurrentLine))?;
        stdout.execute(cursor::MoveLeft(100))?;
    }

    stdout.execute(cursor::Hide)?;
    stdout.execute(cursor::MoveLeft(100))?;
    stdout.execute(cursor::MoveUp(1))?;
    stdout.execute(Clear(ClearType::FromCursorDown))?;

    println!(
        "{question} {} {}",
        "How many results would you like to search for?".bold(),
        input.dimmed(),
    );

    Ok(input.parse()?)
}
