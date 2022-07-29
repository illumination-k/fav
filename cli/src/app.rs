use anyhow::Result;

use crossterm::event::{self, Event, KeyCode};
use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

use crate::highlighter::CodeHighligher;
pub enum Mode {
    Normal,
    Editing,
}

pub struct App {
    input: String,
    mode: Mode,
    messages: Vec<String>,
}

impl Default for App {
    fn default() -> Self {
        App {
            input: String::new(),
            mode: Mode::Normal,
            messages: Vec::new(),
        }
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<()> {
    loop {
        terminal.draw(|f| { ui(f, &app).unwrap(); })?;

        if let Event::Key(key) = event::read()? {
            match app.mode {
                Mode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        app.mode = Mode::Editing;
                    }
                    KeyCode::Char('q') => return Ok(()),
                    _ => {}
                },
                Mode::Editing => match key.code {
                    KeyCode::Enter => app.messages.push(app.input.drain(..).collect()),
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => {
                        app.mode = Mode::Normal;
                    }
                    _ => {}
                },
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) -> Result<()> {
    let highligher = CodeHighligher::default();
    let chunks = Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    let (msg, style) = match app.mode {
        Mode::Normal => (
            vec![Span::raw("Normal")],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        Mode::Editing => (vec![Span::raw("Edit")], Style::default()),
    };

    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);

    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let input = Paragraph::new(app.input.as_ref())
        .style(match app.mode {
            Mode::Normal => Style::default(),
            Mode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));

    f.render_widget(input, chunks[1]);

    match app.mode {
        Mode::Normal => {}
        Mode::Editing => f.set_cursor(chunks[1].x + app.input.width() as u16 + 1, chunks[1].y + 1),
    }

    let messages = app
        .messages
        .iter()
        .map(|m| {
            let content = highligher.highlight("rs", m);
            Ok(ListItem::new(content?))
        })
        .collect::<Result<Vec<ListItem>>>()?;

    let messages =
        List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
    f.render_widget(messages, chunks[2]);
    Ok(())
}
