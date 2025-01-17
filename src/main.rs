use std::io::{self, BufRead, Write};

mod handlers;
mod db;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    db::main::start(); // Initializing the database

    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut is_auth = false;
    let users = db::orm::user::get_all()?;

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

                let _ = db::orm::user::create("root", password_data);
                is_auth = true;
            } else {
                print!("</> Type your password: ");
                io::stdout().flush()?; // Forcefully output a string to the console

                // Reading the input line
                let mut input = String::new();
                handle.read_line(&mut input)?;
                let input = input.trim();

                // Check password
                if input == users[0].password {
                    is_auth = true;
                    println!("PASS-MAN | Authentication successful!");
                } else {
                    println!("PASS-MAN | Incorrect password. Try again.");
                    continue;
                }
            }
        }

        // После аутентификации вызываем функцию для обработки команд
        handlers::handle_commands(&mut handle);
    }
}
