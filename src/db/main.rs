use once_cell::sync::Lazy;
use rusqlite::{Connection, Result};
use std::sync::Mutex;

// Глобальный объект для подключения к базе данных
pub static DB_CONNECTION: Lazy<Mutex<Connection>> = Lazy::new(|| {
    let conn = Connection::open("database.db").expect("ERROR | Error getting connection");
    Mutex::new(conn)
});

/// Функция для инициализации базы данных (создание таблицы)
fn init_database() -> Result<()> {
    let conn = DB_CONNECTION.lock().expect("ERROR | Error getting connection");
    
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS users (
            id        INTEGER PRIMARY KEY AUTOINCREMENT,
            login     TEXT NOT NULL UNIQUE,
            password  TEXT NOT NULL,
            encode_key TEXT NOT NULL,
            encode_iv TEXT NOT NULL
        )
        ",
        [],
    )?;
    
    
    // println!("INFO | Table 'users' successfully created or already exists.");

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS passwords (
            id        INTEGER PRIMARY KEY AUTOINCREMENT,
            platform  TEXT NOT NULL,
            password  TEXT NOT NULL,
            user_id   INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        )
        ",
        [],
    )?;

    // println!("INFO | Table 'passwords' successfully created or already exists.");
    
    Ok(())
}

pub fn start() {
    // Инициализация базы данных
    if let Err(err) = init_database() {
        eprintln!("ERROR | Error initializing database: {}", err);
        return;
    }
}
