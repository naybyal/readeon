use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct Note {
    pub id: i64,
    pub book_id: i64,
    pub page: Option<u32>,
    pub quote: Option<String>,
    pub note: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct NoteTag {
    pub note_id: i64,
    pub tag_id: i64,
}
