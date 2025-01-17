use crate::db::main::DB_CONNECTION;

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub login: String,
    pub password: String,
}

pub fn create(login: &str, password: &str) -> Result<(), rusqlite::Error> {
	let conn = DB_CONNECTION.lock().expect("ERROR | Error getting connection");
    
	conn.execute(
			"
			INSERT INTO users (login, password) 
			VALUES (?1, ?2)
			",
			[login, password],
	)?;
	
	// println!("INFO | User '{}' was created.", login);
	Ok(())

}

pub fn get_all() -> Result<Vec<User>, rusqlite::Error> {
	let conn = DB_CONNECTION.lock().expect("ERROR | Error getting connection");
    
	let mut stmt = conn.prepare("SELECT id, login, password FROM users")?;
	let users = stmt
			.query_map([], |row| {
					Ok(User {
							id: row.get(0)?,
							login: row.get(1)?,
							password: row.get(2)?,
					})
			})?
			.filter_map(|user| user.ok()) // Игнорируем некорректные строки
			.collect();
	
	Ok(users)
}