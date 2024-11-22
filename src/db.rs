use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

pub async fn initialize_database() -> Result<Pool<Sqlite>, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
    .connect("sqlite:C:\\Users\\peppe\\Desktop\\Progetti\\rustAuth\\src\\users.db")
    .await;

    match pool {
        Ok(pool) => {
            println!("Connessione al database riuscita");
            // Creazione tabella
            sqlx::query(
                "CREATE TABLE IF NOT EXISTS users (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    email TEXT UNIQUE NOT NULL,
                    password TEXT NOT NULL
                )",
            )
            .execute(&pool)
            .await
            .expect("Errore durante la creazione della tabella");

            Ok(pool)
        }
        Err(e) => {
            println!("Errore nella connessione al database: {}", e);
            println!("Debugging: Connessione al database fallita con l'errore: {:?}", e);
            Err(e)
        }
    }
}
