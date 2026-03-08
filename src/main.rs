pub mod domain;
pub mod db;
pub mod api;
pub mod app;
pub mod ui;

use rusqlite::Connection;
use std::error::Error;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;
use app::{AppState, Screen};
use db::init::init_db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let conn = init_db("readeon.db")?;

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app_state = AppState::new();
    ui::library_view::load_books(&mut app_state, &conn)?;

    // App loop
    let res = run_app(&mut terminal, &mut app_state, &conn).await;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

use std::io::Stdout;

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut AppState,
    conn: &Connection,
) -> Result<(), Box<dyn Error>> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if let Event::Key(key) = event::read()? {
            match app.screen {
                Screen::Library => ui::library_view::handle_event(key, app, conn).await?,
                Screen::BookDetail => ui::book_view::handle_event(key, app, conn).await?,
                Screen::AddBook => ui::add_book_view::handle_event(key, app, conn).await?,
                Screen::AddSession => ui::add_session_view::handle_event(key, app, conn).await?,
                Screen::AddNote => ui::add_note_view::handle_event(key, app, conn).await?,
                Screen::Stats => ui::stats_view::handle_event(key, app, conn).await?,
            }
        }

        if app.quit {
            return Ok(());
        }
    }
}
