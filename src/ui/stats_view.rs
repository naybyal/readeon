use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};
use crate::app::{AppState, Screen};
use rusqlite::Connection;
use crossterm::event::{KeyEvent, KeyCode};
use std::error::Error;

pub fn draw(f: &mut Frame, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0)].as_ref())
        .split(f.area());

    if let Some(stats) = &app.current_stats {
        let text = vec![
            Line::from(format!("Total Books: {}", stats.total_books)),
            Line::from(format!("Finished: {}", stats.books_finished)),
            Line::from(""),
            Line::from(format!("Total Pages Read: {}", stats.total_pages_read)),
            Line::from(format!("Total Reading Time: {} minutes", stats.total_minutes_read)),
            Line::from(format!("Avg Speed: {:.1} pages/hour", stats.average_pages_per_hour)),
            Line::from(""),
            Line::from("Press 'q' or 'Esc' to return to Library"),
        ];

        let block = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("Reading Statistics"));
        f.render_widget(block, chunks[0]);
    }
}

pub async fn handle_event(key: KeyEvent, app: &mut AppState, _conn: &Connection) -> Result<(), Box<dyn Error>> { 
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => {
            app.screen = Screen::Library;
        }
        _ => {}
    }
    Ok(()) 
}
