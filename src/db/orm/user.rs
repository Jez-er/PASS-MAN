use crate::db::main::DB_CONNECTION;
use crate::utils::encoder;

#[derive(Debug)]
pub struct User {
    // pub id: i32,
    // pub login: String,
    pub password: String,
		pub encode_key: String,
    pub encode_iv: String,
}

pub fn create(login: &str, password: &str) -> Result<(), rusqlite::Error> {
	let conn = DB_CONNECTION.lock().expect("ERROR | Error getting connection");
  
	let key = encoder::generate_key();
	let encoded_key = encoder::encode_key(&key);
	let iv = encoder::generate_iv();
	let encoded_iv = encoder::encode_iv(&iv);

	conn.execute(
			"
			INSERT INTO users (login, password, encode_key, encode_iv) 
			VALUES (?1, ?2, ?3, ?4)
			",
			[login, password, &encoded_key, &encoded_iv],
	)?;
	
	// println!("INFO | User '{}' was created.", login);
	Ok(())

}

pub fn get_all() -> Result<Vec<User>, rusqlite::Error> {
	let conn = DB_CONNECTION.lock().expect("ERROR | Error getting connection");
    
	let mut stmt = conn.prepare("SELECT id, login, password, encode_key, encode_iv FROM users")?;
	let users = stmt
			.query_map([], |row| {
					Ok(User {
							// id: row.get(0)?,
							// login: row.get(1)?,
							password: row.get(2)?,
							encode_key: row.get(3)?,
							encode_iv: row.get(4)?,
					})
			})?
			.filter_map(|user| user.ok()) // Ignore invalid lines
			.collect();
	
	Ok(users)
}

pub fn get_by_id(user_id: i32) -> Result<Option<User>, rusqlite::Error> {
	let conn = crate::db::main::DB_CONNECTION.lock().expect("Error getting connection");

	let mut stmt = conn.prepare("SELECT id, login, password, encode_key, encode_iv FROM users WHERE id = ?")?;
	let user_iter = stmt.query_map([user_id], |row| {
			Ok(User {
					// id: row.get(0)?,
					// login: row.get(1)?,
					password: row.get(2)?,
					encode_key: row.get(3)?,
					encode_iv: row.get(4)?,
			})
	})?;

	// Return the first user (if found) or None if there is no user with this id
	let user = user_iter.into_iter().next().transpose()?;
	Ok(user)
}