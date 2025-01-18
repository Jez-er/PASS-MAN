use rusqlite::OptionalExtension;

use crate::db::main::DB_CONNECTION;
use crate::utils::encoder;

pub fn add(user_id: i64, user_iv: &[u8], user_key: &[u8], platform: &str, password: &str) -> Result<(), Box<dyn std::error::Error>> {
	let conn = DB_CONNECTION.lock().expect("ERROR | Error getting connection");

    let encoded_password = encoder::encrypt_password(password, user_key, user_iv).unwrap();


	conn.execute(
			"INSERT INTO passwords (platform, password, user_id) VALUES (?1, ?2, ?3)",
			rusqlite::params![platform, encoded_password, user_id], // Use the `params!` macro
	)?;
	Ok(())
}

pub fn get_by_platform(
    user_id: i64,
    platform: &str,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let conn = DB_CONNECTION.lock().expect("ERROR | Error getting connection");

    let mut stmt = conn.prepare(
        "SELECT password FROM passwords WHERE user_id = ?1 AND platform = ?2",
    )?;

    // Execute the request and process the result
    let password: Option<String> = stmt
        .query_row(rusqlite::params![user_id, platform], |row| row.get(0))
        .optional()?; // Return `None` if nothing found

    Ok(password)
}

