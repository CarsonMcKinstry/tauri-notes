use super::schema::notes;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = notes)]
pub struct Note {
    pub id: Vec<u8>,
    pub title: String,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub active: bool,
}

impl Note {
    pub fn new(title: String, content: String) -> Self {
        Note {
            id: Uuid::new_v4().as_bytes().to_vec(),
            title,
            content,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
            active: true,
        }
    }
}
