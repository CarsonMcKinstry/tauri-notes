use juniper::{graphql_object, FieldResult};

use super::{
    context::Context,
    models::{note::Note, Human},
};

pub(super) struct Query;

#[graphql_object]
#[graphql(context = Context)]
impl Query {
    fn api_version() -> &'static str {
        "1.0"
    }

    fn human(human_id: String) -> FieldResult<Human> {
        Ok(Human {
            id: human_id,
            name: "Luke Skywalker".into(),
            appears_in: Vec::new(),
            home_planet: "Tatoine".into(),
        })
    }

    fn notes(context: &Context) -> FieldResult<Vec<Note>> {
        let connection = &mut context.pool.get()?;

        let results = Note::get_notes(connection)?;

        Ok(results)
    }
}
