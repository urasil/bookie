use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::fs;

pub async fn setup_database() -> Result<SqlitePool, sqlx::Error>{
    if !std::path::Path::new("db.sqlite").exists() {
        fs::File::create("db.sqlite").expect("Couldn't create db file");
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:db.sqlite")
        .await?;

    sqlx::query(
    "
    CREATE TABLE IF NOT EXISTS places (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            image TEXT NOT NULL,
            description TEXT NOT NULL,
            price REAL NOT NULL,
            location TEXT NOT NULL,
            liked BOOLEAN NOT NULL DEFAULT 0
    );
    "
    ).execute(&pool).await?;

    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM places")
        .fetch_one(&pool)
        .await?;

    if count == 0 {
        let places = vec![
            ("id1", "Hotel Monaco", "https://images.unsplash.com/photo-1542312386-ef949fb5951d", "A luxurious hotel in the heart of the city.", 250.0, "New York"),
            ("id2", "Cozy Cottage", "https://images.unsplash.com/photo-1571004169970-d85c2c776de1", "A rustic cottage by the lake.", 120.0, "Lake Placid"),
            ("id3", "Urban Loft", "https://images.unsplash.com/photo-1517441999911-381655097402", "A modern loft with a great view.", 180.0, "Los Angeles"),
            ("id4", "Beach House", "https://images.unsplash.com/photo-1560942544-78904791e847", "A beautiful house right on the beach.", 300.0, "Miami"),
            ("id5", "Mountain Retreat", "https://images.unsplash.com/photo-1518296238865-c7e6c9a35d79", "A secluded retreat in the mountains.", 200.0, "Aspen"),
        ];

        for place in places {
            sqlx::query("INSERT INTO places (id, name, image, description, price, location) VALUES (?, ?, ?, ?, ?, ?)")
            .bind(place.0) 
            .bind(place.1)
            .bind(place.2)
            .bind(place.3)
            .bind(place.4)
            .bind(place.5)
            .execute(&pool)
            .await?;

        }
    }
    Ok(pool)
}

