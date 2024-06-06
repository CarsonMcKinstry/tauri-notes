use juniper::{graphql_object, FieldResult};

use super::context::Context;
use crate::models::note::*;

pub(super) struct Mutation;

#[graphql_object]
#[graphql(context = Context)]
impl Mutation {
    fn create_note(context: &Context, data: NoteCreateInput) -> FieldResult<Note> {
        let connection = &mut context.pool.get()?;

        let results = Note::create(connection, data)?;
        Ok(results)
    }
}
