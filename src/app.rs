use crate::domain::{book::Book, session::ReadingSession, note::Note, stats::ReadingStats};

#[derive(Default)]
pub struct AddBookState {
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub pages: String,
    pub year: String,
    pub focus: usize, // 0: title, 1: author, 2: isbn, 3: pages, 4: year
    pub status_msg: String,
}

#[derive(Default)]
pub struct AddSessionState {
    pub minutes: String,
    pub pages: String,
    pub focus: usize, // 0: minutes, 1: pages
    pub status_msg: String,
}

#[derive(Default)]
pub struct AddNoteState {
    pub page: String,
    pub quote: String,
    pub note: String,
    pub focus: usize, // 0: page, 1: quote, 2: note
    pub status_msg: String,
}

pub enum Screen {
    Library,
    BookDetail,
    AddBook,
    AddSession,
    AddNote,
    Stats,
}

pub struct AppState {
    pub screen: Screen,
    pub books: Vec<Book>,
    pub selected_book_index: usize,
    pub current_sessions: Vec<ReadingSession>,
    pub current_notes: Vec<Note>,
    pub current_stats: Option<ReadingStats>,
    pub add_book_state: AddBookState,
    pub add_session_state: AddSessionState,
    pub add_note_state: AddNoteState,
    pub quit: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            screen: Screen::Library,
            books: Vec::new(),
            selected_book_index: 0,
            current_sessions: Vec::new(),
            current_notes: Vec::new(),
            current_stats: None,
            add_book_state: Default::default(),
            add_session_state: Default::default(),
            add_note_state: Default::default(),
            quit: false,
        }
    }
}
