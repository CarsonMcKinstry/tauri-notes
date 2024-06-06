// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate diesel;
extern crate dotenv;

use std::path::PathBuf;

use juniper::{DefaultScalarValue, ExecutionError, GraphQLError, Value, Variables};
use tauri::{AppHandle, Manager};
use tauri_notes::db::setup_database;
use tauri_notes::graphql::execute;

fn get_app_data_dir(app_handle: AppHandle) -> PathBuf {
    app_handle
        .path_resolver()
        .app_data_dir()
        .expect("Unable to get app data directory")
}

#[tauri::command]
fn graphql(
    app_handle: AppHandle,
    query: &str,
    operation_name: Option<&str>,
    variables: Option<Variables>,
) -> Result<(Value, Vec<ExecutionError<DefaultScalarValue>>), GraphQLError> {
    let data_dir = get_app_data_dir(app_handle);
    execute(query, operation_name, variables, data_dir)
}

fn main() {
    let _ = fix_path_env::fix();
    tauri::Builder::default()
        .setup(|app| {
            let data_dir = get_app_data_dir(app.handle());
            setup_database(data_dir);

            #[cfg(debug_assertions)]
            app.get_window("main").unwrap().open_devtools();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![graphql])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
