use std::io::{self, BufRead, Write};

mod commands;

pub fn handle_commands(handle: &mut dyn BufRead) {
    print!("</> enter command: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    handle.read_line(&mut input).unwrap();
    let input = input.trim();

    match input {
        "shell" => {
            commands::shell::activate();
        }
        "exit" | "q" => {
            commands::exit::activate();
        }
        _ => {
            println!("PASS-MAN | Unknown command: {}", input);
        }
    }
}
