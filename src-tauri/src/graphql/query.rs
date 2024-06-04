use juniper::graphql_object;

use super::context::Context;

pub(super) struct Query;

#[graphql_object]
#[graphql(context = Context)]
impl Query {
    fn api_version() -> &'static str {
        "1.0"
    }
}
