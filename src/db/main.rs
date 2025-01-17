use once_cell::sync::Lazy;
use rusqlite::{Connection, Result};
use std::sync::Mutex;

// Глобальный объект для подключения к базе данных
pub static DB_CONNECTION: Lazy<Mutex<Connection>> = Lazy::new(|| {
    let conn = Connection::open("database.db").expect("Ошибка подключения к базе данных");
    Mutex::new(conn)
});

/// Функция для инициализации базы данных (создание таблицы)
fn init_database() -> Result<()> {
    let conn = DB_CONNECTION.lock().expect("Ошибка получения соединения");
    
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS users (
            id      INTEGER PRIMARY KEY AUTOINCREMENT,
            login   TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL
        )
        ",
        [],
    )?;
    
    println!("Таблица 'users' успешно создана или уже существует.");
    Ok(())
}

pub fn start() {
    // Инициализация базы данных
    if let Err(err) = init_database() {
        eprintln!("Ошибка при инициализации базы данных: {}", err);
        return;
    }
}
