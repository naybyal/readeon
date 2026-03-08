use rusqlite::{Connection, Result, params};
use crate::domain::book::{Book, BookStatus};
use chrono::NaiveDate;

pub struct BookRepository<'a> {
    pub conn: &'a Connection,
}

impl<'a> BookRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn insert(&self, book: &Book) -> Result<i64> {
        let status_str = match book.status {
            BookStatus::Unread => "unread",
            BookStatus::Reading => "reading",
            BookStatus::Finished => "finished",
            BookStatus::Abandoned => "abandoned",
        };

        self.conn.execute(
            "INSERT INTO books (title, author, isbn, pages, year, status, rating, start_date, finish_date, current_page)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                book.title,
                book.author,
                book.isbn,
                book.pages,
                book.year,
                status_str,
                book.rating,
                book.start_date.map(|d| d.format("%Y-%m-%d").to_string()),
                book.finish_date.map(|d| d.format("%Y-%m-%d").to_string()),
                book.current_page
            ],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn fetch_all(&self) -> Result<Vec<Book>> {
        let mut stmt = self.conn.prepare("SELECT id, title, author, isbn, pages, year, status, rating, start_date, finish_date, current_page FROM books")?;
        let book_iter = stmt.query_map([], |row| {
            let status_str: String = row.get(6)?;
            let status = match status_str.as_str() {
                "reading" => BookStatus::Reading,
                "finished" => BookStatus::Finished,
                "abandoned" => BookStatus::Abandoned,
                _ => BookStatus::Unread,
            };

            let start_date_str: Option<String> = row.get(8)?;
            let start_date = start_date_str.and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());

            let finish_date_str: Option<String> = row.get(9)?;
            let finish_date = finish_date_str.and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());

            Ok(Book {
                id: row.get(0)?,
                title: row.get(1)?,
                author: row.get(2)?,
                isbn: row.get(3)?,
                pages: row.get(4)?,
                year: row.get(5)?,
                status,
                rating: row.get(7)?,
                start_date,
                finish_date,
                current_page: row.get(10)?,
            })
        })?;

        let mut books = Vec::new();
        for book in book_iter {
            books.push(book?);
        }
        Ok(books)
    }

    pub fn delete(&self, id: i64) -> Result<()> {
        self.conn.execute("DELETE FROM books WHERE id = ?1", [id])?;
        Ok(())
    }

    pub fn update_progress(&self, id: i64, current_page: u32, status: BookStatus, finish_date: Option<NaiveDate>) -> Result<()> {
        let status_str = match status {
            BookStatus::Unread => "unread",
            BookStatus::Reading => "reading",
            BookStatus::Finished => "finished",
            BookStatus::Abandoned => "abandoned",
        };
        let finish_date_str = finish_date.map(|d| d.format("%Y-%m-%d").to_string());

        self.conn.execute(
            "UPDATE books SET current_page = ?1, status = ?2, finish_date = ?3 WHERE id = ?4",
            params![current_page, status_str, finish_date_str, id]
        )?;
        Ok(())
    }
}
