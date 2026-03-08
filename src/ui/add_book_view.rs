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
use crate::domain::book::{Book, BookStatus};

pub fn draw(f: &mut Frame, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(14), Constraint::Min(0)].as_ref())
        .split(f.area());
        
    let state = &app.add_book_state;

    let text = vec![
        Line::from(vec![
            Span::styled("Title:  ", Style::default()),
            Span::styled(&state.title, if state.focus == 0 { Style::default().bg(Color::DarkGray) } else { Style::default() })
        ]),
        Line::from(vec![
            Span::styled("Author: ", Style::default()),
            Span::styled(&state.author, if state.focus == 1 { Style::default().bg(Color::DarkGray) } else { Style::default() })
        ]),
        Line::from(vec![
            Span::styled("ISBN:   ", Style::default()),
            Span::styled(&state.isbn, if state.focus == 2 { Style::default().bg(Color::DarkGray) } else { Style::default() })
        ]),
        Line::from(vec![
            Span::styled("Pages:  ", Style::default()),
            Span::styled(&state.pages, if state.focus == 3 { Style::default().bg(Color::DarkGray) } else { Style::default() })
        ]),
        Line::from(vec![
            Span::styled("Year:   ", Style::default()),
            Span::styled(&state.year, if state.focus == 4 { Style::default().bg(Color::DarkGray) } else { Style::default() })
        ]),
        Line::default(),
        Line::from("[Enter] Save  |  [Esc] Cancel  |  [Ctrl+F] Fetch ISBN"),
        Line::styled(&state.status_msg, Style::default().fg(Color::Yellow)),
    ];

    let block = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Add Book"));
    
    f.render_widget(block, chunks[0]);
}

pub async fn handle_event(key: KeyEvent, app: &mut AppState, conn: &Connection) -> Result<(), Box<dyn Error>> { 
    match key.code {
        KeyCode::Esc => {
            app.screen = Screen::Library;
            app.add_book_state = Default::default(); // reset
        }
        KeyCode::Tab | KeyCode::Down => {
            app.add_book_state.focus = (app.add_book_state.focus + 1) % 5;
        }
        KeyCode::BackTab | KeyCode::Up => {
            app.add_book_state.focus = (app.add_book_state.focus + 4) % 5;
        }
        KeyCode::Char('f') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            let isbn = app.add_book_state.isbn.clone();
            app.add_book_state.status_msg = "Fetching...".to_string();
            if let Ok(Some(meta)) = crate::api::openlibrary::fetch_by_isbn(&isbn).await {
                app.add_book_state.title = meta.title;
                if !meta.authors.is_empty() {
                    app.add_book_state.author = meta.authors[0].clone();
                }
                if let Some(p) = meta.pages {
                    app.add_book_state.pages = p.to_string();
                }
                if let Some(y) = meta.publish_year {
                    app.add_book_state.year = y.to_string();
                }
                app.add_book_state.status_msg = "Metadata loaded. Please verify.".to_string();
            } else {
                app.add_book_state.status_msg = "Failed to fetch metadata".to_string();
            }
        }
        KeyCode::Enter => {
            // Save
            let title = app.add_book_state.title.trim().to_string();
            let author = app.add_book_state.author.trim().to_string();
            if title.is_empty() || author.is_empty() {
                app.add_book_state.status_msg = "Title and Author are required".to_string();
                return Ok(());
            }

            let book = Book {
                id: 0,
                title,
                author,
                isbn: if app.add_book_state.isbn.trim().is_empty() { None } else { Some(app.add_book_state.isbn.trim().to_string()) },
                pages: app.add_book_state.pages.trim().parse().ok(),
                year: app.add_book_state.year.trim().parse().ok(),
                status: BookStatus::Unread,
                rating: None,
                start_date: None,
                finish_date: None,
                current_page: 0,
            };

            let repo = crate::db::book_repo::BookRepository::new(conn);
            if let Err(e) = repo.insert(&book) {
                app.add_book_state.status_msg = format!("Error: {}", e);
            } else {
                crate::ui::library_view::load_books(app, conn)?;
                app.screen = Screen::Library;
                app.add_book_state = Default::default();
            }
        }
        KeyCode::Backspace => {
            match app.add_book_state.focus {
                0 => { app.add_book_state.title.pop(); }
                1 => { app.add_book_state.author.pop(); }
                2 => { app.add_book_state.isbn.pop(); }
                3 => { app.add_book_state.pages.pop(); }
                4 => { app.add_book_state.year.pop(); }
                _ => {}
            }
        }
        KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
            match app.add_book_state.focus {
                0 => app.add_book_state.title.push(c),
                1 => app.add_book_state.author.push(c),
                2 => app.add_book_state.isbn.push(c),
                3 => app.add_book_state.pages.push(c),
                4 => app.add_book_state.year.push(c),
                _ => {}
            }
        }
        _ => {}
    }
    Ok(()) 
}
