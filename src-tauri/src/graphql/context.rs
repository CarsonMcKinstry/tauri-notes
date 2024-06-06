use std::path::PathBuf;

use crate::db::{establish_connection, DbPool};

pub(super) struct Context {
    #[allow(dead_code)]
    pub pool: DbPool,
}

impl juniper::Context for Context {}

impl Context {
    fn with_pool(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn from_data_dir(data_dir: PathBuf) -> Self {
        let pool = establish_connection(data_dir);

        Self::with_pool(pool)
    }
}
