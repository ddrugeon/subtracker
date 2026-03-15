mod app;
mod config;
mod db;
mod models;
mod services;
mod ui;

fn main() {
    let database_path = config::get_database_path().unwrap();
    let conn = db::migration::open_database(database_path.as_path()).unwrap();
    db::migration::run_migrations(&conn).unwrap();
}
