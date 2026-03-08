use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct ReadingSession {
    pub id: i64,
    pub book_id: i64,
    pub date: NaiveDate,
    pub minutes_read: u32,
    pub pages_read: u32,
}

impl ReadingSession {
    pub fn pages_per_hour(&self) -> f32 {
        let hours = self.minutes_read as f32 / 60.0;
        if hours == 0.0 {
            return 0.0;
        }
        self.pages_read as f32 / hours
    }
}
