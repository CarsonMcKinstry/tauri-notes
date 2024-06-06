use juniper::{graphql_object, FieldResult};

use super::context::Context;
use crate::models::note::*;

pub struct Mutation;

#[graphql_object]
#[graphql(context = Context)]
impl Mutation {
    fn create_note(context: &Context, data: NoteCreateInput) -> FieldResult<Note> {
        let connection = &mut context.pool.get()?;

        let results = Note::create(data, connection)?;
        Ok(results)
    }

    fn edit_note(context: &Context, note_id: String, data: NoteEditInput) -> FieldResult<Note> {
        let connection = &mut context.pool.get()?;

        let results = Note::update(note_id, data, connection)?;

        Ok(results)
    }

    fn delete_note(context: &Context, note_id: String) -> FieldResult<bool> {
        let connection = &mut context.pool.get()?;

        let results = Note::delete(note_id, connection)?;

        Ok(results)
    }
}
