use std::io::{self, BufRead, Write};

mod commands;

pub fn handle_commands(handle: &mut dyn BufRead) {

    let simple_pass_length: usize = 9;
    let pass_length: usize = 12;


    print!("</> enter command: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    handle.read_line(&mut input).unwrap();
    let input = input.trim();

    match input {
        "gen_pass" | "gp" => {
            commands::pass_gen::activate(pass_length);
        }
        "gen_simple" | "gs" => {
            commands::simple_pass_gen::activate(simple_pass_length);
        }
        "exit" | "q" => {
            commands::exit::activate();
        }
        _ => {
            println!("PASS-MAN | Unknown command: {}", input);
        }
    }
}
