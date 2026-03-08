use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use crate::app::{AppState, Screen};
use crate::db::book_repo::BookRepository;
use rusqlite::Connection;
use crossterm::event::{KeyEvent, KeyCode};
use std::error::Error;

pub fn load_books(app: &mut AppState, conn: &Connection) -> Result<(), Box<dyn Error>> {
    let repo = BookRepository::new(conn);
    app.books = repo.fetch_all()?;
    if app.books.is_empty() {
        app.selected_book_index = 0;
    } else if app.selected_book_index >= app.books.len() {
        app.selected_book_index = app.books.len() - 1;
    }
    Ok(())
}

pub fn draw(f: &mut Frame, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3)
        ].as_ref())
        .split(f.area());

    // Header
    let header = Paragraph::new("READEON")
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    // Main area split
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(60)
        ].as_ref())
        .split(chunks[1]);

    // List of books
    let items: Vec<ListItem> = app.books.iter().enumerate().map(|(i, b)| {
        let prefix = if i == app.selected_book_index { "> " } else { "  " };
        let content = format!("{}{}", prefix, b.title);
        let style = if i == app.selected_book_index {
            Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan)
        } else {
            Style::default()
        };
        ListItem::new(Line::from(Span::styled(content, style)))
    }).collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Library"));
    
    f.render_widget(list, main_chunks[0]);

    // Book Detail preview
    if let Some(book) = app.books.get(app.selected_book_index) {
        let text = vec![
            Line::from(format!("Author: {}", book.author)),
            Line::from(format!("Pages: {}", book.pages.unwrap_or(0))),
            Line::from(format!("Status: {:?}", book.status)),
            Line::from(format!("Progress: {} / {}", book.current_page, book.pages.unwrap_or(0))),
        ];
        let detail = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title(format!("Book Details: {}", book.title)));
        f.render_widget(detail, main_chunks[1]);
    } else {
        let detail = Paragraph::new("No books yet. Press 'a' to add one.")
            .block(Block::default().borders(Borders::ALL).title("Book Details"));
        f.render_widget(detail, main_chunks[1]);
    }

    // Footer stats
    let total = app.books.len();
    let stats = Paragraph::new(format!("Stats: Books {}", total))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(stats, chunks[2]);
}

pub async fn handle_event(key: KeyEvent, app: &mut AppState, conn: &Connection) -> Result<(), Box<dyn Error>> { 
    match key.code {
        KeyCode::Char('q') => app.quit = true,
        KeyCode::Char('j') | KeyCode::Down => {
            if !app.books.is_empty() && app.selected_book_index < app.books.len() - 1 {
                app.selected_book_index += 1;
            }
        }
        KeyCode::Char('k') | KeyCode::Up => {
            if app.selected_book_index > 0 {
                app.selected_book_index -= 1;
            }
        }
        KeyCode::Char('a') => {
            app.screen = Screen::AddBook;
        }
        KeyCode::Char('s') => {
            let stats_repo = crate::db::stats_repo::StatsRepository::new(conn);
            app.current_stats = stats_repo.fetch_stats().ok();
            app.screen = Screen::Stats;
        }
        KeyCode::Enter => {
            if !app.books.is_empty() {
                if let Some(book) = app.books.get(app.selected_book_index) {
                    let session_repo = crate::db::session_repo::SessionRepository::new(conn);
                    let note_repo = crate::db::note_repo::NoteRepository::new(conn);
                    app.current_sessions = session_repo.fetch_by_book(book.id).unwrap_or_default();
                    app.current_notes = note_repo.fetch_by_book(book.id).unwrap_or_default();
                }
                app.screen = Screen::BookDetail;
            }
        }
        _ => {}
    }
    Ok(()) 
}
