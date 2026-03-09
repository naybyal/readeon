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
use crate::domain::session::ReadingSession;
use chrono::Local;

pub fn draw(f: &mut Frame, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(8), Constraint::Min(0)].as_ref())
        .split(f.area());
        
    let state = &app.add_session_state;

    let text = vec![
        Line::from(vec![
            Span::styled("Minutes Read: ", Style::default()),
            Span::styled(&state.minutes, if state.focus == 0 { Style::default().bg(Color::DarkGray) } else { Style::default() })
        ]),
        Line::from(vec![
            Span::styled("Pages Read:   ", Style::default()),
            Span::styled(&state.pages, if state.focus == 1 { Style::default().bg(Color::DarkGray) } else { Style::default() })
        ]),
        Line::default(),
        Line::from("[Enter] Save  |  [Esc] Cancel"),
        Line::styled(&state.status_msg, Style::default().fg(Color::Yellow)),
    ];

    let title = if state.id.is_some() { "Edit Reading Session" } else { "Add Reading Session" };
    let block = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title(title));
    
    f.render_widget(block, chunks[0]);
}

pub async fn handle_event(key: KeyEvent, app: &mut AppState, conn: &Connection) -> Result<(), Box<dyn Error>> { 
    match key.code {
        KeyCode::Esc => {
            app.screen = Screen::BookDetail;
            app.add_session_state = Default::default(); // reset
        }
        KeyCode::Tab | KeyCode::Down => {
            app.add_session_state.focus = (app.add_session_state.focus + 1) % 2;
        }
        KeyCode::BackTab | KeyCode::Up => {
            app.add_session_state.focus = (app.add_session_state.focus + 1) % 2;
        }
        KeyCode::Enter => {
            let minutes: u32 = app.add_session_state.minutes.trim().parse().unwrap_or(0);
            let pages: u32 = app.add_session_state.pages.trim().parse().unwrap_or(0);

            if minutes == 0 || pages == 0 {
                app.add_session_state.status_msg = "Must be valid numbers > 0".to_string();
                return Ok(());
            }

            if let Some(book) = app.books.get(app.selected_book_index) {
                let repo = crate::db::session_repo::SessionRepository::new(conn);

                let is_edit = app.add_session_state.id.is_some();
                let session = if let Some(id) = app.add_session_state.id {
                    // Try to find existing to preserve date
                    if let Some(existing) = app.current_sessions.iter().find(|s| s.id == id) {
                        let mut s = existing.clone();
                        s.minutes_read = minutes;
                        s.pages_read = pages;
                        s
                    } else {
                        ReadingSession {
                            id,
                            book_id: book.id,
                            date: Local::now().date_naive(),
                            minutes_read: minutes,
                            pages_read: pages,
                        }
                    }
                } else {
                    ReadingSession {
                        id: 0,
                        book_id: book.id,
                        date: Local::now().date_naive(),
                        minutes_read: minutes,
                        pages_read: pages,
                    }
                };

                let res = if is_edit {
                    repo.update(&session)
                } else {
                    repo.insert(&session).map(|_| ())
                };

                if let Err(e) = res {
                    app.add_session_state.status_msg = format!("Error: {}", e);
                } else {
                    app.current_sessions = repo.fetch_by_book(book.id).unwrap_or_default();
                    app.screen = Screen::BookDetail;
                    app.add_session_state = Default::default();
                    
                    // Also update book progress and potentially status here
                    let new_page = book.current_page + pages;
                    let book_repo = crate::db::book_repo::BookRepository::new(conn);
                    let status = if let Some(tot) = book.pages { 
                        if new_page >= tot { crate::domain::book::BookStatus::Finished } else { crate::domain::book::BookStatus::Reading }
                    } else { 
                        crate::domain::book::BookStatus::Reading 
                    };
                    let finish_date = if status == crate::domain::book::BookStatus::Finished { Some(Local::now().date_naive()) } else { None };
                    
                    let _ = book_repo.update_progress(book.id, new_page, status, finish_date);
                    // Reload books to get the updated status
                    let _ = crate::ui::library_view::load_books(app, conn);
                }
            } else {
                app.screen = Screen::Library;
            }
        }
        KeyCode::Backspace => {
            match app.add_session_state.focus {
                0 => { app.add_session_state.minutes.pop(); }
                1 => { app.add_session_state.pages.pop(); }
                _ => {}
            }
        }
        KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
            if c.is_ascii_digit() {
                match app.add_session_state.focus {
                    0 => app.add_session_state.minutes.push(c),
                    1 => app.add_session_state.pages.push(c),
                    _ => {}
                }
            }
        }
        _ => {}
    }
    Ok(()) 
}
