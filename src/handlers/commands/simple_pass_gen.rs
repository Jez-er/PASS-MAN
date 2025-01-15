use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;


pub fn activate(length: usize)  {
	let pass = thread_rng()
		.sample_iter(&Alphanumeric)
		.take(length)
		.map(char::from)
		.collect::<String>();

	println!("PASS-MAN | Generated password: {}", pass);
 	// Копируем пароль в буфер обмена
	let mut clipboard: ClipboardContext = ClipboardContext::new().unwrap();
 	clipboard.set_contents(pass.clone()).unwrap();
 	println!("PASS-MAN | Password copied to clipboard! ✔️");
}
