use std::io::{self, BufRead, Write};

mod commands;

pub fn handle_commands(handle: &mut dyn BufRead) {
    let pass_length: usize = 12;

    print!("</> enter command: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    handle.read_line(&mut input).unwrap();
    let input = input.trim();

    // Parse the command and parameters
    let parts: Vec<&str> = input.split_whitespace().collect();
    let command = parts.get(0).map(|&s| s).unwrap_or(""); // The first part is the command
    let params = &parts[1..]; // The remaining parts are parameters

    // Flags and parameters
    let mut pass_gen_command_is_simple = false;
    let mut length = match command {
        "gen_pass" | "gp" => pass_length,
        _ => 0,
    };

    let mut flags: Vec<&str> = Vec::new();
    let mut platform = String::new();
    let mut password = String::new();

    // Check flags and parameters
    let mut params_iter = params.iter();
    while let Some(param) = params_iter.next() {
        if param.starts_with("--") || param.starts_with("-") {
            flags.push(param);
        } else if let Ok(parsed_length) = param.parse::<usize>() {
            length = parsed_length; // Convert the length parameter
        } else if platform.is_empty() {
            platform = param.to_string(); // The first parameter is the platform
        } else if password.is_empty() {
            password = param.to_string(); // The second parameter is the password
        }
    }

    match command {
        "gen_pass" | "gp" => {
            for flag in &flags {
                match *flag {
                    "--simple" | "-s" => pass_gen_command_is_simple = true,
                    _ => println!("PASS-MAN | Unknown flag: {}", flag),
                }
            }
            if pass_gen_command_is_simple {
                commands::simple_pass_gen::activate(length);
            } else {
                commands::pass_gen::activate(length); // Pass the flag and length
            }
        }
        "save" | "s" => {
            // Check for the presence of two parameters for the save command
            if params.len() == 2 {
                platform = params[0].to_string();
                password = params[1].to_string();
                commands::save::activate(&platform, &password); // Save the password
            } else {
                println!("PASS-MAN | Usage: save <platform> <password>");
            }
        }
        "get" | "g" => {
            // Handle flags for the get command
            let mut is_show = false; // Example flag for verbose output
            for flag in &flags {
                match *flag {
                    "--show" | "-s" => is_show = true,
                    _ => println!("PASS-MAN | Unknown flag: {}", flag),
                }
            }

            // Check for the presence of one parameter (platform)
            if params.len() >= 1 {
                platform = params[0].to_string();
                commands::get::activate(&platform, is_show)
            } else {
                println!("PASS-MAN | Usage: get <platform> [--verbose]");
            }
        }
        "list" | "l" => {
            commands::list::activate()
        }
        "exit" | "q" => {
            commands::exit::activate();
        }
        _ => {
            println!("PASS-MAN | Unknown command: {}", input);
        }
    }
}
