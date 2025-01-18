use crate::db::orm::passwords;
use crate::utils::get_user_data;
use crate::utils::encoder;

pub fn activate(platform_data: &str, password_data: &str) {
    // Get encode_key and encode_iv using the encode() function
    if let Some((encode_key, encode_iv)) = get_user_data::encode() {

				// Decode IV and key
				let iv_data = encoder::decode_iv(&encode_iv).unwrap();
				let key_data = encoder::decode_key(&encode_key).unwrap();

        match passwords::add(1, &iv_data, &key_data, platform_data, password_data,) {
            Ok(_) => println!("PASS-MAN | Password for the platform '{}' successfully added. ✔️", platform_data),
            Err(err) => eprintln!("PASS-MAN | Failed to add password: {}", err),
        }
    } else {
        eprintln!("PASS-MAN | Failed to retrieve user encoding data.");
    }
}
