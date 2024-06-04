// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use graphql::execute;
use juniper::{DefaultScalarValue, ExecutionError, GraphQLError, Value, Variables};
use tauri::Manager;

mod graphql;

#[tauri::command]
fn graphql(
    query: &str,
    operation_name: Option<&str>,
    variables: Option<Variables>,
) -> Result<(Value, Vec<ExecutionError<DefaultScalarValue>>), GraphQLError> {
    execute(query, operation_name, variables)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            app.get_window("main").unwrap().open_devtools();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![graphql])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
