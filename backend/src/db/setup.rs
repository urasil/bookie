use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::fs;

pub async fn setup_database() -> Result<SqlitePool, sqlx::Error> {
    if !std::path::Path::new("db.sqlite").exists() {
        fs::File::create("db.sqlite").expect("Couldn't create db file");
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:db.sqlite")
        .await?;

    sqlx::query(
        "
       CREATE TABLE IF NOT EXISTS users (
              id TEXT PRIMARY KEY, 
              name TEXT NOT NULL,
              email TEXT NOT NULL UNIQUE
       );

    ",
    )
    .execute(&pool)
    .await?;

    // User Likes Table
    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS user_likes (
            user_id TEXT NOT NULL,
            place_id TEXT NOT NULL,
            PRIMARY KEY (user_id, place_id),
            FOREIGN KEY (user_id) REFERENCES users(id)
            FOREIGN KEY (place_id) REFERENCES places(id)
        );
        ",
    )
    .execute(&pool)
    .await?;

    // Dummy user for testing, remove in prod
    sqlx::query("INSERT OR IGNORE INTO users (id, name, email) VALUES ('test_user', 'Test User', 'test@example.com')")
        .execute(&pool)
        .await?;

    Ok(pool)
}
