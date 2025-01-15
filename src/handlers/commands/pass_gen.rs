use rand::seq::{IteratorRandom, SliceRandom};
use rand::thread_rng;
use rand::distributions::{Alphanumeric, Distribution};

pub fn activate(length: usize) {
    let special_chars = "!@#$%^&*()-_=+[]{};:,.<>?";

    let mut rng = thread_rng();

    // Генерация символов: буквы и цифры
    let charset: Vec<char> = Alphanumeric
        .sample_iter(&mut rng)
        .take(length - 1)  // Оставляем место для спецсимвола
        .map(char::from)
        .collect();

    // Добавляем хотя бы один спецсимвол
    let mut password: String = charset.into_iter().collect();
    let special_char: char = special_chars.chars().choose(&mut rng).unwrap();  // Выбираем случайный спецсимвол
    password.push(special_char);

    // Перемешиваем символы
    let mut password_chars: Vec<char> = password.chars().collect();
    password_chars.shuffle(&mut rng);
    let final_password: String = password_chars.into_iter().collect();

    println!("PASS-MAN | Generated password: {}", final_password);
}