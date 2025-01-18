use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

use crate::db::orm::passwords;
use crate::utils::get_user_data;
use crate::utils::encoder;

pub fn activate(platform: &str, is_show: bool) {
    let user_id = 1; // Example user ID

    match passwords::get_by_platform(user_id, platform) {
        Ok(Some(encrypted_password)) => {
            // Get encode_key and encode_iv using the encode() function
            if let Some((encode_key, encode_iv)) = get_user_data::encode() {
                // Decode IV and key
                let iv_data = encoder::decode_iv(&encode_iv).unwrap();
                let key_data = encoder::decode_key(&encode_key).unwrap();

                // Decrypt the password
                match encoder::decrypt_password(&encrypted_password, &key_data, &iv_data) {
                    Ok(decrypted_password) => {
                        if is_show {
                            println!("PASS-MAN | Password for platform '{}' is: {}", platform, decrypted_password);
                            
                            let mut clipboard: ClipboardContext = ClipboardContext::new().unwrap();
                            clipboard.set_contents(decrypted_password.clone()).unwrap();
                            println!("PASS-MAN | Password copied to clipboard! ✔️");
                        } else {
                            let mut clipboard: ClipboardContext = ClipboardContext::new().unwrap();
                            clipboard.set_contents(decrypted_password.clone()).unwrap();
                            println!("PASS-MAN | Password for platform '{}' copied to clipboard! ✔️", platform);
                        }
                    }
                    Err(err) => {
                        eprintln!("ERROR | Failed to decrypt password: {}", err);
                    }
                }
            } else {
                eprintln!("PASS-MAN | Failed to retrieve user encoding data.");
            }
        }
        Ok(None) => {
            println!(
                "PASS-MAN | No password found for platform '{}'",
                platform
            );
        }
        Err(err) => {
            eprintln!("ERROR | Failed to retrieve password: {}", err);
        }
    }
}
