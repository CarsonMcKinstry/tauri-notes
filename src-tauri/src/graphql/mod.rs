mod context;
mod query;

use std::path::PathBuf;

use context::Context;

use juniper::{
    DefaultScalarValue, EmptyMutation, EmptySubscription, ExecutionError, GraphQLError, RootNode,
    Value, Variables,
};
use query::Query;

#[allow(dead_code)]
type Schema = juniper::RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

#[allow(dead_code)]
pub(crate) fn execute(
    query: &str,
    operation_name: Option<&str>,
    variables: Option<Variables>,
    data_dir: PathBuf,
) -> Result<(Value, Vec<ExecutionError<DefaultScalarValue>>), GraphQLError> {
    let ctx = Context::from_data_dir(data_dir);

    let variables = variables.unwrap_or(Variables::new());

    juniper::execute_sync(
        query,
        operation_name,
        &Schema::new(Query, EmptyMutation::new(), EmptySubscription::new()),
        &variables,
        &ctx,
    )
}

#[allow(dead_code)]
pub fn get_sdl() -> String {
    let schema = RootNode::new(
        Query,
        EmptyMutation::<()>::new(),
        EmptySubscription::<()>::new(),
    );

    schema.as_sdl()
}
