use rusqlite::{Connection, Result};
use crate::domain::stats::ReadingStats;

pub struct StatsRepository<'a> {
    pub conn: &'a Connection,
}

impl<'a> StatsRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn fetch_stats(&self) -> Result<ReadingStats> {
        // Some sums could be NULL if there are no sessions. unwrap_or(0) handles this in logic usually but rusqlite gives Error on NULL if decoding u32 directly.
        // COALESCE solves this.
        let total_books: usize = self.conn.query_row("SELECT COUNT(*) FROM books", [], |row| {
            let val: i64 = row.get(0)?;
            Ok(val as usize)
        }).unwrap_or(0);
        let books_finished: usize = self.conn.query_row("SELECT COUNT(*) FROM books WHERE status = 'finished'", [], |row| {
            let val: i64 = row.get(0)?;
            Ok(val as usize)
        }).unwrap_or(0);
        let total_pages_read: u32 = self.conn.query_row("SELECT COALESCE(SUM(pages_read), 0) FROM reading_sessions", [], |row| row.get(0)).unwrap_or(0);
        let total_minutes_read: u32 = self.conn.query_row("SELECT COALESCE(SUM(minutes_read), 0) FROM reading_sessions", [], |row| row.get(0)).unwrap_or(0);
        
        // Coalesce the floating point too
        let avg_pages_val: f64 = self.conn.query_row(
            "SELECT COALESCE(SUM(pages_read) * 60.0 / SUM(minutes_read), 0.0) FROM reading_sessions;",
            [],
            |row| row.get(0)
        ).unwrap_or(0.0);

        Ok(ReadingStats {
            total_books,
            books_finished,
            total_pages_read,
            total_minutes_read,
            average_pages_per_hour: avg_pages_val as f32,
        })
    }
}
