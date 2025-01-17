use rand::seq::{IteratorRandom, SliceRandom};
use rand::thread_rng;
use rand::distributions::{Alphanumeric, Distribution};
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

pub fn activate(length: usize) {
    let special_chars = "!@#$%^&*()-_=+[]{};:,.<>?";

    let mut rng = thread_rng();

    // Генерация символов: буквы и цифры
    let charset: Vec<char> = Alphanumeric
        .sample_iter(&mut rng)
        .take(length - 1)  // Leave space for a special character
        .map(char::from)
        .collect();

    // Add at least one special character
    let mut password: String = charset.into_iter().collect();
    let special_char: char = special_chars.chars().choose(&mut rng).unwrap();  // Select a random special character
    password.push(special_char);

    // Mix the symbols
    let mut password_chars: Vec<char> = password.chars().collect();
    password_chars.shuffle(&mut rng);
    let final_password: String = password_chars.into_iter().collect();

    println!("PASS-MAN | Generated password: {}", final_password);

    // Copy the password to the clipboard
    let mut clipboard: ClipboardContext = ClipboardContext::new().unwrap();
    clipboard.set_contents(final_password.clone()).unwrap();
    println!("PASS-MAN | Password copied to clipboard! ✔️");
}
