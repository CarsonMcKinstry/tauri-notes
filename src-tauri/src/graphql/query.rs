use juniper::{graphql_object, FieldResult};

use super::context::Context;
use crate::models::note::*;

pub(super) struct Query;

#[graphql_object]
#[graphql(context = Context)]
impl Query {
    fn api_version() -> &'static str {
        "1.0"
    }

    fn notes(context: &Context) -> FieldResult<Vec<Note>> {
        let connection = &mut context.pool.get()?;

        let results = Note::get_notes(connection)?;

        Ok(results)
    }
}
