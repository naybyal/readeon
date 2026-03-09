use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, List, ListItem},
};
use crate::app::{AppState, Screen};
use rusqlite::Connection;
use crossterm::event::{KeyEvent, KeyCode};
use std::error::Error;

pub fn draw(f: &mut Frame, app: &AppState) {
    let book = match app.books.get(app.selected_book_index) {
        Some(b) => b,
        None => return,
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ].as_ref())
        .split(f.area());

    // Book Info
    let info = vec![
        Line::from(format!("Author: {}", book.author)),
        Line::from(format!("Pages: {}", book.pages.unwrap_or(0))),
        Line::from(format!("Status: {:?}", book.status)),
        Line::from(format!("Progress: {} / {}", book.current_page, book.pages.unwrap_or(0))),
    ];
    let book_info = Paragraph::new(info)
        .block(Block::default().borders(Borders::ALL).title(format!("Book: {}", book.title)));
    f.render_widget(book_info, chunks[0]);

    // Sessions List
    let session_items: Vec<ListItem> = app.current_sessions.iter().enumerate().map(|(i, s)| {
        let prefix = if i == app.selected_session_index && app.book_view_focus == 1 { "> " } else { "  " };
        let content = format!("{}{} | {} min | {} pages", prefix, s.date.format("%Y-%m-%d"), s.minutes_read, s.pages_read);
        let style = if i == app.selected_session_index && app.book_view_focus == 1 { Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan) } else { Style::default() };
        ListItem::new(Line::from(Span::styled(content, style)))
    }).collect();
    let border_style_sessions = if app.book_view_focus == 1 { Style::default().fg(Color::Cyan) } else { Style::default() };
    let sessions_list = List::new(session_items)
        .block(Block::default().borders(Borders::ALL).border_style(border_style_sessions).title("Reading Sessions (r: Add, e: Edit, D: Del, Tab: Switch)"));
    f.render_widget(sessions_list, chunks[1]);

    // Notes List
    let note_items: Vec<ListItem> = app.current_notes.iter().enumerate().map(|(i, n)| {
        let prefix = if i == app.selected_note_index && app.book_view_focus == 2 { "> " } else { "  " };
        let page_str = n.page.map(|p| format!("Page {} - ", p)).unwrap_or_default();
        let content = format!("{}{}{}", prefix, page_str, n.note);
        let style = if i == app.selected_note_index && app.book_view_focus == 2 { Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan) } else { Style::default() };
        ListItem::new(Line::from(Span::styled(content, style)))
    }).collect();
    let border_style_notes = if app.book_view_focus == 2 { Style::default().fg(Color::Cyan) } else { Style::default() };
    let notes_list = List::new(note_items)
        .block(Block::default().borders(Borders::ALL).border_style(border_style_notes).title("Notes (n: Add, e: Edit, D: Del)"));
    f.render_widget(notes_list, chunks[2]);
}

pub async fn handle_event(key: KeyEvent, app: &mut AppState, conn: &Connection) -> Result<(), Box<dyn Error>> { 
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => {
            app.screen = Screen::Library;
        }
        KeyCode::Tab => {
            app.book_view_focus = match app.book_view_focus {
                0 => 1,
                1 => 2,
                _ => 0,
            };
            if app.book_view_focus == 1 && app.selected_session_index >= app.current_sessions.len() && !app.current_sessions.is_empty() {
                app.selected_session_index = app.current_sessions.len() - 1;
            }
            if app.book_view_focus == 2 && app.selected_note_index >= app.current_notes.len() && !app.current_notes.is_empty() {
                app.selected_note_index = app.current_notes.len() - 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if app.book_view_focus == 1 && !app.current_sessions.is_empty() && app.selected_session_index < app.current_sessions.len() - 1 {
                app.selected_session_index += 1;
            } else if app.book_view_focus == 2 && !app.current_notes.is_empty() && app.selected_note_index < app.current_notes.len() - 1 {
                app.selected_note_index += 1;
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if app.book_view_focus == 1 && app.selected_session_index > 0 {
                app.selected_session_index -= 1;
            } else if app.book_view_focus == 2 && app.selected_note_index > 0 {
                app.selected_note_index -= 1;
            }
        }
        KeyCode::Char('e') => {
            if app.book_view_focus == 1 && !app.current_sessions.is_empty() {
                if let Some(session) = app.current_sessions.get(app.selected_session_index) {
                    app.add_session_state.id = Some(session.id);
                    app.add_session_state.minutes = session.minutes_read.to_string();
                    app.add_session_state.pages = session.pages_read.to_string();
                    app.add_session_state.focus = 0;
                    app.add_session_state.status_msg = String::new();
                    app.screen = Screen::AddSession;
                }
            } else if app.book_view_focus == 2 && !app.current_notes.is_empty() {
                if let Some(note) = app.current_notes.get(app.selected_note_index) {
                    app.add_note_state.id = Some(note.id);
                    app.add_note_state.page = note.page.map(|p| p.to_string()).unwrap_or_default();
                    app.add_note_state.quote = note.quote.clone().unwrap_or_default();
                    app.add_note_state.note = note.note.clone();
                    app.add_note_state.focus = 0;
                    app.add_note_state.status_msg = String::new();
                    app.screen = Screen::AddNote;
                }
            }
        }
        KeyCode::Char('D') | KeyCode::Delete => {
            if let Some(book) = app.books.get(app.selected_book_index) {
                if app.book_view_focus == 1 && !app.current_sessions.is_empty() {
                    if let Some(session) = app.current_sessions.get(app.selected_session_index) {
                        let repo = crate::db::session_repo::SessionRepository::new(conn);
                        if repo.delete(session.id).is_ok() {
                            app.current_sessions = repo.fetch_by_book(book.id).unwrap_or_default();
                            if app.selected_session_index >= app.current_sessions.len() && !app.current_sessions.is_empty() {
                                app.selected_session_index = app.current_sessions.len() - 1;
                            } else if app.current_sessions.is_empty() {
                                app.selected_session_index = 0;
                            }
                        }
                    }
                } else if app.book_view_focus == 2 && !app.current_notes.is_empty() {
                    if let Some(note) = app.current_notes.get(app.selected_note_index) {
                        let repo = crate::db::note_repo::NoteRepository::new(conn);
                        if repo.delete(note.id).is_ok() {
                            app.current_notes = repo.fetch_by_book(book.id).unwrap_or_default();
                            if app.selected_note_index >= app.current_notes.len() && !app.current_notes.is_empty() {
                                app.selected_note_index = app.current_notes.len() - 1;
                            } else if app.current_notes.is_empty() {
                                app.selected_note_index = 0;
                            }
                        }
                    }
                }
            }
        }
        KeyCode::Char('p') => { /* update progress */ }
        KeyCode::Char('r') => {
            app.screen = Screen::AddSession;
            app.add_session_state = Default::default();
        }
        KeyCode::Char('n') => {
            app.screen = Screen::AddNote;
            app.add_note_state = Default::default();
        }
        KeyCode::Char('d') => { /* delete book */ }
        _ => {}
    }
    Ok(()) 
}
