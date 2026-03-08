use super::{book::{Book, BookStatus}, session::ReadingSession};

#[derive(Debug)]
pub struct ReadingStats {
    pub total_books: usize,
    pub books_finished: usize,
    pub total_pages_read: u32,
    pub total_minutes_read: u32,
    pub average_pages_per_hour: f32,
}

impl ReadingStats {
    pub fn compute(sessions: &[ReadingSession], books: &[Book]) -> Self {
        let total_minutes: u32 = sessions.iter().map(|s| s.minutes_read).sum();
        let total_pages: u32 = sessions.iter().map(|s| s.pages_read).sum();

        let pages_per_hour = if total_minutes == 0 {
            0.0
        } else {
            total_pages as f32 / (total_minutes as f32 / 60.0)
        };

        let finished_books = books
            .iter()
            .filter(|b| matches!(b.status, BookStatus::Finished))
            .count();

        Self {
            total_books: books.len(),
            books_finished: finished_books,
            total_pages_read: total_pages,
            total_minutes_read: total_minutes,
            average_pages_per_hour: pages_per_hour,
        }
    }
}
