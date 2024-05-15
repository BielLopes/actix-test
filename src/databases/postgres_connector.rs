use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn start_connection() -> Pool<Postgres> {
    let postgres_enviroment = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&postgres_enviroment)
        .await
        .expect("Failed to connect to Postgres");

    match sqlx::migrate!("./src/databases/migrations")
        .run(&pool)
        .await {
        Ok(_) => println!("Migrations ran successfully"),
        Err(e) => println!("Error running migrations: {:?}", e),
    }

    pool
}