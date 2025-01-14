use std::io::{self, BufRead, Write};

mod handlers;

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut is_auth = false;

    loop {
        // Если пользователь не аутентифицирован, просим ввести пароль
        if !is_auth {
            print!("</> Type your password: ");
            io::stdout().flush().unwrap();

            // Чтение строки ввода
            let mut input = String::new();
            handle.read_line(&mut input).unwrap();
            let input = input.trim();

            if input == "123" {
                is_auth = true;
                println!("PASS-MAN | Authentication successful!");
            } else {
                println!("PASS-MAN | Incorrect password. Try again.");
                continue;
            }
        }

        // После аутентификации вызываем функцию для обработки команд
        handlers::handle_commands(&mut handle);
    }
}
