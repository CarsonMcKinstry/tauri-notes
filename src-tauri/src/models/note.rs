use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use juniper::{GraphQLInputObject, GraphQLObject};
use uuid::Uuid;

use crate::schema::notes;

#[derive(GraphQLObject, Queryable, Insertable, Clone)]
#[diesel(table_name = notes)]
pub struct Note {
    id: String,
    title: String,
    content: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    active: bool,
}

#[derive(GraphQLInputObject)]

pub struct NoteCreateInput {
    title: String,
    content: Option<String>,
}

impl Note {
    fn from_input(input: &NoteCreateInput) -> Self {
        let now_utc = Utc::now();
        let now_naive: NaiveDateTime = now_utc.naive_utc();

        Note {
            id: Uuid::new_v4().to_string(),
            title: input.title.clone(),
            content: input.content.clone().unwrap_or("".into()),
            created_at: now_naive,
            updated_at: now_naive,
            active: true,
        }
    }

    pub fn get_notes(connection: &mut SqliteConnection) -> QueryResult<Vec<Note>> {
        use crate::schema::notes::dsl::*;

        notes
            .filter(active.eq(true))
            .order(updated_at.desc())
            .load::<Note>(connection)
    }

    pub fn get_note(note_id: String, connection: &mut SqliteConnection) -> QueryResult<Note> {
        use crate::schema::notes::dsl::*;

        notes.find(note_id).get_result(connection)
    }

    pub fn create(connection: &mut SqliteConnection, data: NoteCreateInput) -> QueryResult<Note> {
        let note = Note::from_input(&data);

        diesel::insert_into(notes::table)
            .values(&note)
            .execute(connection)?;

        Note::get_note(note.id, connection)
    }
}
