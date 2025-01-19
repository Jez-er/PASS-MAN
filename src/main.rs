use std::io::{self, BufRead, Write};
use bcrypt::{hash, verify, DEFAULT_COST};

mod handlers;
mod utils;
mod db;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    db::main::start(); // Initializing the database

    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut is_auth = false;
    let users = db::orm::user::get_all().unwrap();

    loop {
        if !is_auth {
            if users.is_empty() {
                println!("PASS-MAN | It looks like the user doesn't exist yet.");
                println!("PASS-MAN | Let's create it now!");


                print!("</> Type password for pass-man: ");
                io::stdout().flush()?; // Forcefully output a string to the console
                // Reading the input line
                let mut password = String::new();
                handle.read_line(&mut password)?;
                let password_data: &str = password.trim();
                
                let hashed_password = match hash(password_data, DEFAULT_COST) {
                    Ok(h) => h,
                    Err(e) => {
                        eprintln!("Ошибка при хешировании пароля: {}", e);
                        return Ok(());
                    }
                };

                let _ = db::orm::user::create("root", &hashed_password);
                is_auth = true;
                handlers::handle_commands(&mut handle);

            } else {
                print!("</> Type your password: ");
                io::stdout().flush()?; // Forcefully output a string to the console

                // Reading the input line
                let mut input = String::new();
                handle.read_line(&mut input)?;
                let input = input.trim();

                match verify(input, &users[0].password) {
                    Ok(true) => {
                        is_auth = true;
                        println!("PASS-MAN | Authentication successful!");
                        handlers::handle_commands(&mut handle);
                    }
                    Ok(false) => {
                        println!("PASS-MAN | Incorrect password. Try again.");
                        continue;
                    }
                    Err(e) => eprintln!("Ошибка при проверке пароля: {}", e),
                }
            }
        }
    }
}
