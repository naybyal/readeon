use ratatui::Frame;
use crate::app::{AppState, Screen};

pub mod library_view;
pub mod book_view;
pub mod add_book_view;
pub mod stats_view;

pub fn draw(f: &mut Frame, app: &AppState) {
    match app.screen {
        Screen::Library => library_view::draw(f, app),
        Screen::BookDetail => book_view::draw(f, app),
        Screen::AddBook => add_book_view::draw(f, app),
        Screen::Stats => stats_view::draw(f, app),
    }
}
