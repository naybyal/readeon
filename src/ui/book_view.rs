use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    text::Line,
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
    let session_items: Vec<ListItem> = app.current_sessions.iter().map(|s| {
        ListItem::new(Line::from(format!("{} | {} min | {} pages", s.date.format("%Y-%m-%d"), s.minutes_read, s.pages_read)))
    }).collect();
    let sessions_list = List::new(session_items)
        .block(Block::default().borders(Borders::ALL).title("Reading Sessions (r: Add log)"));
    f.render_widget(sessions_list, chunks[1]);

    // Notes List
    let note_items: Vec<ListItem> = app.current_notes.iter().map(|n| {
        let page_str = n.page.map(|p| format!("Page {} - ", p)).unwrap_or_default();
        ListItem::new(Line::from(format!("{}{}", page_str, n.note)))
    }).collect();
    let notes_list = List::new(note_items)
        .block(Block::default().borders(Borders::ALL).title("Notes (n: Add note)"));
    f.render_widget(notes_list, chunks[2]);
}

pub async fn handle_event(key: KeyEvent, app: &mut AppState, _conn: &Connection) -> Result<(), Box<dyn Error>> { 
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => {
            app.screen = Screen::Library;
        }
        KeyCode::Char('p') => { /* update progress */ }
        KeyCode::Char('r') => { /* log session */ }
        KeyCode::Char('n') => { /* add note */ }
        KeyCode::Char('d') => { /* delete book */ }
        _ => {}
    }
    Ok(()) 
}
