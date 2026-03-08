use chrono::NaiveDate;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BookStatus {
    Unread,
    Reading,
    Finished,
    Abandoned,
}

#[derive(Debug, Clone)]
pub struct Book {
    pub id: i64,
    pub title: String,
    pub author: String,
    pub isbn: Option<String>,
    pub pages: Option<u32>,
    pub year: Option<u32>,
    pub status: BookStatus,
    pub rating: Option<u8>,
    pub start_date: Option<NaiveDate>,
    pub finish_date: Option<NaiveDate>,
    pub current_page: u32,
}

impl Book {
    pub fn progress_percent(&self) -> Option<f32> {
        match self.pages {
            Some(total) if total > 0 => {
                Some((self.current_page as f32 / total as f32) * 100.0)
            }
            _ => None,
        }
    }

    pub fn start_reading(&mut self, date: NaiveDate) {
        self.status = BookStatus::Reading;
        self.start_date = Some(date);
    }

    pub fn finish(&mut self, date: NaiveDate) {
        self.status = BookStatus::Finished;
        self.finish_date = Some(date);
    }
}
