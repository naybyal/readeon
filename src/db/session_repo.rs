use rusqlite::{Connection, Result, params};
use crate::domain::session::ReadingSession;
use chrono::NaiveDate;

pub struct SessionRepository<'a> {
    pub conn: &'a Connection,
}

impl<'a> SessionRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn insert(&self, session: &ReadingSession) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO reading_sessions (book_id, date, minutes_read, pages_read)
             VALUES (?1, ?2, ?3, ?4)",
            params![
                session.book_id,
                session.date.format("%Y-%m-%d").to_string(),
                session.minutes_read,
                session.pages_read
            ],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn fetch_by_book(&self, book_id: i64) -> Result<Vec<ReadingSession>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, book_id, date, minutes_read, pages_read FROM reading_sessions WHERE book_id = ?1 ORDER BY date DESC"
        )?;

        let iter = stmt.query_map([book_id], |row| {
            let date_str: String = row.get(2)?;
            let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").unwrap_or_default();

            Ok(ReadingSession {
                id: row.get(0)?,
                book_id: row.get(1)?,
                date,
                minutes_read: row.get(3)?,
                pages_read: row.get(4)?,
            })
        })?;

        let mut sessions = Vec::new();
        for session in iter {
            sessions.push(session?);
        }
        Ok(sessions)
    }
}
