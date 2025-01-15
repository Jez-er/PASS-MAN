use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn activate(length: usize)  {
	let pass = thread_rng()
		.sample_iter(&Alphanumeric)
		.take(length)
		.map(char::from)
		.collect::<String>();

println!("PASS-MAN | Generated password: {}", pass);
}
