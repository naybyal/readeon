use rusqlite::{Connection, Result, params};
use crate::domain::note::Note;
use chrono::NaiveDateTime;

pub struct NoteRepository<'a> {
    pub conn: &'a Connection,
}

impl<'a> NoteRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn insert(&self, note: &Note) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO notes (book_id, page, quote, note, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                note.book_id,
                note.page,
                note.quote,
                note.note,
                note.created_at.format("%Y-%m-%d %H:%M:%S").to_string()
            ],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn fetch_by_book(&self, book_id: i64) -> Result<Vec<Note>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, book_id, page, quote, note, created_at FROM notes WHERE book_id = ?1 ORDER BY created_at DESC"
        )?;

        let iter = stmt.query_map([book_id], |row| {
            let date_str: String = row.get(5)?;
            let created_at = NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S").unwrap_or_default();
            Ok(Note {
                id: row.get(0)?,
                book_id: row.get(1)?,
                page: row.get(2)?,
                quote: row.get(3)?,
                note: row.get(4)?,
                created_at,
            })
        })?;

        let mut notes = Vec::new();
        for n in iter {
            notes.push(n?);
        }
        Ok(notes)
    }

    pub fn update(&self, note: &Note) -> Result<()> {
        self.conn.execute(
            "UPDATE notes SET page = ?1, quote = ?2, note = ?3 WHERE id = ?4",
            params![
                note.page,
                note.quote,
                note.note,
                note.id
            ],
        )?;
        Ok(())
    }

    pub fn delete(&self, id: i64) -> Result<()> {
        self.conn.execute(
            "DELETE FROM notes WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }
}
