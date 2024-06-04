use std::fs::File;
use std::io::prelude::*;

use tauri_notes::graphql::get_sdl;

fn main() -> std::io::Result<()> {
    let mut file = File::create("../schema.graphql")?;
    file.write_all(get_sdl().as_bytes())?;

    Ok(())
}
