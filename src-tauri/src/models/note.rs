use chrono::{NaiveDateTime, Utc};
use diesel::{prelude::*, update};
use juniper::{graphql_object, FieldError, FieldResult, GraphQLInputObject, Value};
use uuid::Uuid;

use crate::schema::notes;

#[derive(Queryable, Selectable, Insertable, Clone)]
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

#[derive(GraphQLInputObject)]
pub struct NoteEditInput {
    title: Option<String>,
    content: Option<String>,
}

#[derive(AsChangeset)]
#[diesel(table_name = notes)]
pub struct NoteChangeSet {
    title: Option<String>,
    content: Option<String>,
    updated_at: Option<NaiveDateTime>,
}

impl NoteEditInput {
    fn to_change_set(self) -> NoteChangeSet {
        NoteChangeSet {
            title: self.title,
            content: self.content,
            updated_at: Some(Utc::now().naive_utc()),
        }
    }
}

pub struct NotesResults {
    results: Vec<Note>,
    count: i64,
}

#[graphql_object]
impl NotesResults {
    fn results(&self) -> &Vec<Note> {
        &self.results
    }

    fn count(&self) -> FieldResult<i32> {
        self.count
            .try_into()
            .map_err(|_| FieldError::new("User count exceeds i32 range", Value::null()))
    }
}

#[graphql_object]
impl Note {
    fn id(&self) -> &str {
        &self.id
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn content(&self) -> &str {
        &self.content
    }

    fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }

    fn updated_at(&self) -> &NaiveDateTime {
        &self.updated_at
    }

    fn active(&self) -> bool {
        self.active
    }

    fn summary(&self) -> &str {
        "summary"
    }
}

impl Note {
    fn from_input(input: &NoteCreateInput) -> Self {
        let now_naive = Utc::now().naive_utc();

        Note {
            id: Uuid::new_v4().to_string(),
            title: input.title.clone(),
            content: input.content.clone().unwrap_or_default(),
            created_at: now_naive,
            updated_at: now_naive,
            active: true,
        }
    }

    pub fn get_notes(connection: &mut SqliteConnection) -> QueryResult<NotesResults> {
        use crate::schema::notes::dsl::*;

        let count = notes
            .filter(active.eq(true))
            .count()
            .get_result(connection)?;
        let results = notes
            .filter(active.eq(true))
            .order(updated_at.desc())
            .load::<Note>(connection)?;

        Ok(NotesResults { results, count })
    }

    pub fn get_note(note_id: String, connection: &mut SqliteConnection) -> QueryResult<Note> {
        use crate::schema::notes::dsl::*;
        notes.find(note_id).get_result(connection)
    }

    pub fn create(data: NoteCreateInput, connection: &mut SqliteConnection) -> QueryResult<Note> {
        let note = Note::from_input(&data);

        diesel::insert_into(notes::table)
            .values(&note)
            .returning(Note::as_returning())
            .get_result(connection)
    }

    pub fn update(
        note_id: String,
        data: NoteEditInput,
        connection: &mut SqliteConnection,
    ) -> QueryResult<Note> {
        use crate::schema::notes::dsl::*;

        update(notes)
            .filter(id.eq(note_id))
            .set(data.to_change_set())
            .returning(Note::as_returning())
            .get_result(connection)
    }

    pub fn delete(note_id: String, connection: &mut SqliteConnection) -> QueryResult<bool> {
        use crate::schema::notes::dsl::*;

        update(notes)
            .filter(id.eq(note_id))
            .set(active.eq(false))
            .execute(connection)
            .map(|count| count > 0)
    }
}
