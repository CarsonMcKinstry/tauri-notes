use std::path::PathBuf;

use rocket::{response::content::RawHtml, routes, State};

use tauri_notes::graphql::{create_schema, Context, Schema};

#[rocket::get("/")]
async fn homepage() -> RawHtml<&'static str> {
    RawHtml(
        "<html><h1>juniper_rocket/simple example</h1>\
               <div>visit <a href=\"/graphiql\">GraphiQL</a></div>\
               <div>visit <a href=\"/playground\">GraphQL Playground</a></div>\
         </html>",
    )
}

#[rocket::get("/graphiql")]
fn graphiql() -> RawHtml<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}

#[rocket::get("/playground")]
fn playground() -> RawHtml<String> {
    juniper_rocket::playground_source("/graphql", None)
}

// GET request accepts query parameters like these:
// ?query=<urlencoded-graphql-query-string>
// &operationName=<optional-name>
// &variables=<optional-json-encoded-variables>
// See details here: https://graphql.org/learn/serving-over-http#get-request
#[rocket::get("/graphql?<request..>")]
async fn get_graphql(
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    let path = PathBuf::new().join("");
    request.execute(schema, &Context::from_data_dir(path)).await
}

#[rocket::post("/graphql", data = "<request>")]
async fn post_graphql(
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    let path = PathBuf::new().join("");
    request.execute(schema, &Context::from_data_dir(path)).await
}

#[rocket::main]
async fn main() {
    _ = rocket::build()
        .manage(create_schema())
        .mount(
            "/",
            routes![homepage, graphiql, playground, get_graphql, post_graphql],
        )
        .launch()
        .await
        .expect("server to launch");
}
