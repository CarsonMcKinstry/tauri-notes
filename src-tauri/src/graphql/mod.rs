pub(crate) mod context;
pub(crate) mod mutation;
pub(crate) mod query;

use std::path::PathBuf;

use juniper::{
    DefaultScalarValue, EmptySubscription, ExecutionError, GraphQLError, Value, Variables,
};

use context::Context;
use mutation::Mutation;
use query::Query;

#[allow(dead_code)]
type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {}, EmptySubscription::new())
}

#[allow(dead_code)]
pub fn execute(
    query: &str,
    operation_name: Option<&str>,
    variables: Option<Variables>,
    data_dir: PathBuf,
) -> Result<(Value, Vec<ExecutionError<DefaultScalarValue>>), GraphQLError> {
    let ctx = Context::from_data_dir(data_dir);

    let variables = variables.unwrap_or(Variables::new());

    juniper::execute_sync(query, operation_name, &create_schema(), &variables, &ctx)
}

#[allow(dead_code)]
pub fn get_sdl() -> String {
    let schema = create_schema();

    schema.as_sdl()
}
