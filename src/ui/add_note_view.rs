use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use crate::app::{AppState, Screen};
use rusqlite::Connection;
use crossterm::event::{KeyEvent, KeyCode, KeyModifiers};
use std::error::Error;
use crate::domain::note::Note;
use chrono::Local;

pub fn draw(f: &mut Frame, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(10), Constraint::Min(0)].as_ref())
        .split(f.area());
        
    let state = &app.add_note_state;

    let text = vec![
        Line::from(vec![
            Span::styled("Page:  ", Style::default()),
            Span::styled(if state.page.is_empty() { " " } else { &state.page }, if state.focus == 0 { Style::default().bg(Color::DarkGray) } else { Style::default() })
        ]),
        Line::from(vec![
            Span::styled("Quote: ", Style::default()),
            Span::styled(if state.quote.is_empty() { " " } else { &state.quote }, if state.focus == 1 { Style::default().bg(Color::DarkGray) } else { Style::default() })
        ]),
        Line::from(vec![
            Span::styled("Note:  ", Style::default()),
            Span::styled(if state.note.is_empty() { " " } else { &state.note }, if state.focus == 2 { Style::default().bg(Color::DarkGray) } else { Style::default() })
        ]),
        Line::default(),
        Line::from("[Enter] Save  |  [Esc] Cancel"),
        Line::styled(&state.status_msg, Style::default().fg(Color::Yellow)),
    ];

    let block = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Add Note"));
    
    f.render_widget(block, chunks[0]);
}

pub async fn handle_event(key: KeyEvent, app: &mut AppState, conn: &Connection) -> Result<(), Box<dyn Error>> { 
    match key.code {
        KeyCode::Esc => {
            app.screen = Screen::BookDetail;
            app.add_note_state = Default::default(); // reset
        }
        KeyCode::Tab | KeyCode::Down => {
            app.add_note_state.focus = (app.add_note_state.focus + 1) % 3;
        }
        KeyCode::BackTab | KeyCode::Up => {
            app.add_note_state.focus = (app.add_note_state.focus + 2) % 3;
        }
        KeyCode::Enter => {
            let page: Option<u32> = app.add_note_state.page.trim().parse().ok();
            let quote = if app.add_note_state.quote.trim().is_empty() { None } else { Some(app.add_note_state.quote.trim().to_string()) };
            let note = if app.add_note_state.note.trim().is_empty() { None } else { Some(app.add_note_state.note.trim().to_string()) };

            if quote.is_none() && note.is_none() {
                app.add_note_state.status_msg = "Must provide a quote or a note".to_string();
                return Ok(());
            }

            if let Some(book) = app.books.get(app.selected_book_index) {
                let new_note = Note {
                    id: 0,
                    book_id: book.id,
                    page,
                    quote,
                    note: note.unwrap_or_default(),
                    created_at: Local::now().naive_local(),
                };

                let repo = crate::db::note_repo::NoteRepository::new(conn);
                if let Err(e) = repo.insert(&new_note) {
                    app.add_note_state.status_msg = format!("Error: {}", e);
                } else {
                    app.current_notes = repo.fetch_by_book(book.id).unwrap_or_default();
                    app.screen = Screen::BookDetail;
                    app.add_note_state = Default::default();
                }
            } else {
                app.screen = Screen::Library;
            }
        }
        KeyCode::Backspace => {
            match app.add_note_state.focus {
                0 => { app.add_note_state.page.pop(); }
                1 => { app.add_note_state.quote.pop(); }
                2 => { app.add_note_state.note.pop(); }
                _ => {}
            }
        }
        KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
            match app.add_note_state.focus {
                0 => { if c.is_ascii_digit() { app.add_note_state.page.push(c) } },
                1 => { app.add_note_state.quote.push(c) },
                2 => { app.add_note_state.note.push(c) },
                _ => {}
            }
        }
        _ => {}
    }
    Ok(()) 
}
