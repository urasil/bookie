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
            ("id1", "Hotel Monaco", "https://excellenceriviera.com/wp-content/uploads/2020/05/Hotel-De-Paris-Monaco-01.jpg", "The Hotel Monaco has the best décor of any hotel I have ever stayed in! It's loud patterned wallpaper, interesting light fixtures in both the rooms and common areas.", 250.0, "New York"),
            ("id2", "Cozy Cottage", "https://excellenceriviera.com/wp-content/uploads/2020/05/Hotel-De-Paris-Monaco-01.jpg", "Kick back and enjoy your vacation or business trip with this beautiful 3 bedroom, 1.5 bath home.", 120.0, "Lake Placid"),
            ("id3", "Urban Loft", "https://excellenceriviera.com/wp-content/uploads/2020/05/Hotel-De-Paris-Monaco-01.jpg", "Staying in this 55 m² apartment, nestled relatively close to Downtown Historic District, guests can relax on a sun terrace.", 180.0, "Los Angeles"),
            ("id4", "Beach House", "https://excellenceriviera.com/wp-content/uploads/2020/05/Hotel-De-Paris-Monaco-01.jpg", "Browse 14,005 authentic Miami beach house stock photos, high-res images, and pictures.", 300.0, "Miami"),
            ("id5", "Mountain Retreat", "https://excellenceriviera.com/wp-content/uploads/2020/05/Hotel-De-Paris-Monaco-01.jpg", "Browse 1,567 Aspen Ski Resort stock photos, pictures and royalty-free images from iStock.", 200.0, "Aspen"),
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

