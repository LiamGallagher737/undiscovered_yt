use crate::app::App;
use crate::discover::Discovery;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Paragraph, Tabs};
use std::io::Stdout;

pub fn render_app(frame: &mut ratatui::Frame<CrosstermBackend<Stdout>>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([
            Constraint::Length(1), // Header
            Constraint::Min(0),    // Content
            Constraint::Length(1), // Footer
        ])
        .horizontal_margin(3)
        .vertical_margin(1)
        .split(frame.size());

    draw_header(frame, app, chunks[0]);
    draw_footer(frame, app, chunks[2]);

    let content_chunk = Layout::default()
        .constraints([Constraint::Min(0)])
        .vertical_margin(1)
        .split(chunks[1])[0];

    draw_results(frame, app, content_chunk);
}

pub fn draw_header(
    frame: &mut ratatui::Frame<CrosstermBackend<Stdout>>,
    app: &mut App,
    area: Rect,
) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(20), Constraint::Min(0)])
        .direction(Direction::Horizontal)
        .split(area);

    let title = Text::styled(
        " Undiscovered YT ",
        Style::default()
            .bg(Color::LightRed)
            .add_modifier(Modifier::BOLD),
    );
    frame.render_widget(Paragraph::new(title), chunks[0]);

    let tabs = Tabs::new::<Span>(Discovery::OPTIONS.iter().map(|t| Span::raw(*t)).collect())
        .style(Style::default().add_modifier(Modifier::DIM))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .remove_modifier(Modifier::DIM),
        )
        .divider("│")
        .select(app.tab);
    frame.render_widget(tabs, chunks[1]);
}

pub fn draw_footer(
    frame: &mut ratatui::Frame<CrosstermBackend<Stdout>>,
    _app: &mut App,
    area: Rect,
) {
    let keybinds = [
        ("1/2/3/4", "select discovery"),
        ("r", "refresh"),
        ("k", "api key"),
        ("esc", "close app"),
    ];

    let mut spans = Vec::with_capacity(keybinds.len() * (keybinds.len() / 2));

    for n in 0..keybinds.len() {
        let keybind = keybinds[n];
        spans.push(Span::styled(
            keybind.0,
            Style::default().add_modifier(Modifier::BOLD | Modifier::DIM),
        ));
        spans.push(" ".into());
        spans.push(Span::styled(
            keybind.1,
            Style::default().add_modifier(Modifier::DIM),
        ));
        if n < keybinds.len() - 1 {
            spans.push(Span::styled(
                " • ",
                Style::default().add_modifier(Modifier::DIM),
            ));
        }
    }

    let line = Line::from(spans);

    frame.render_widget(Paragraph::new(line), area);
}

pub fn draw_results(
    frame: &mut ratatui::Frame<CrosstermBackend<Stdout>>,
    app: &mut App,
    area: Rect,
) {
    let list_chunks = Layout::default()
        .constraints(vec![Constraint::Length(3); 5])
        .split(area);

    for (n, result) in app.results.lock().unwrap().iter().enumerate() {
        let selected = false;

        let style = if selected {
            Style::default().fg(Color::LightRed)
        } else {
            Style::default()
        };

        let _prefix = if selected { " │ " } else { "   " };

        let text = Text {
            lines: vec![
                Line::from(vec![
                    // Span::styled(prefix, style),
                    Span::styled(
                        result
                            .snippet
                            .title
                            .to_owned()
                            .unwrap_or("Unnamed".to_string()),
                        style.add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(vec![
                    // Span::styled(prefix, style),
                    Span::styled("Views: ", style.add_modifier(Modifier::DIM)),
                    Span::styled("0", style.add_modifier(Modifier::DIM)),
                    Span::styled(" • ", style.add_modifier(Modifier::DIM)),
                    Span::styled("Channel: ", style.add_modifier(Modifier::DIM)),
                    Span::styled(
                        result
                            .snippet
                            .channel_title
                            .to_owned()
                            .unwrap_or("anonymous".to_string()),
                        style.add_modifier(Modifier::DIM),
                    ),
                ]),
            ],
        };

        frame.render_widget(Paragraph::new(text), list_chunks[n]);
    }
}
