use diesel::{
    r2d2::{self, ConnectionManager},
    SqliteConnection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;
use std::{env, fs::create_dir_all, path::PathBuf};

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

fn run_migrations(pool: DbPool) {
    pool.get()
        .unwrap()
        .run_pending_migrations(MIGRATIONS)
        .unwrap();
}

fn get_database_path(data_dir: PathBuf) -> String {
    dotenv().ok();

    let database_path = env::var("DATABASE_URL").unwrap_or("db.sqlite".into());

    data_dir
        .join("databases")
        .join(database_path)
        .to_string_lossy()
        .to_string()
}

pub fn establish_connection(data_dir: PathBuf) -> DbPool {
    let app_dir_path = data_dir.clone();

    create_dir_all(app_dir_path.join("databases")).expect("Failed creating app data directory!");

    let db_path = get_database_path(data_dir);

    let manager = ConnectionManager::<SqliteConnection>::new(db_path);

    r2d2::Pool::builder().build(manager).unwrap()
}

pub fn setup_database(data_dir: PathBuf) {
    let pool = establish_connection(data_dir);
    run_migrations(pool);
}
