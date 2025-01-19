use crate::db::orm::passwords;
use crate::utils::get_user_data;
use crate::utils::encoder;

pub fn activate() {
    match passwords::get_all() {
        Ok(passwords_list) => {
            if passwords_list.is_empty() {
                println!("PASS-MAN | No passwords found.");
                return;
            }

            // Print the title centered above the table
            println!("\n{:^49}\n", "PASS-MAN");

            // Print table header
            println!("{}", "┌──────────────────────┬───────────────────────────┐");
            println!("{}", "│ Platform             │ Password                  │");
            println!("{}", "├──────────────────────┼───────────────────────────┤");

            for password_entry in passwords_list {
                let platform = password_entry.platform;
                let encrypted_password = password_entry.password;

                // Get encode_key and encode_iv using the encode() function
                if let Some((encode_key, encode_iv)) = get_user_data::encode() {
                    // Decode IV and key
                    match (
                        encoder::decode_iv(&encode_iv),
                        encoder::decode_key(&encode_key),
                    ) {
                        (Ok(iv_data), Ok(key_data)) => {
                            // Decrypt the password
                            match encoder::decrypt_password(&encrypted_password, &key_data, &iv_data) {
                                Ok(decrypted_password) => {
                                    println!(
                                        "│ {:<20} │ {:<25} │",
                                        platform,
                                        decrypted_password
                                    );
                                    println!("{}", "├──────────────────────┼───────────────────────────┤");
                                }
                                Err(err) => {
                                    eprintln!(
                                        "ERROR | Failed to decrypt password for '{}': {}",
                                        platform, err
                                    );
                                }
                            }
                        }
                        (Err(err_iv), Err(err_key)) => {
                            eprintln!(
                                "ERROR | Failed to decode key and IV for '{}': IV: {}, Key: {}",
                                platform, err_iv, err_key
                            );
                        }
                        (Err(err_iv), _) => {
                            eprintln!(
                                "ERROR | Failed to decode IV for '{}': {}",
                                platform, err_iv
                            );
                        }
                        (_, Err(err_key)) => {
                            eprintln!(
                                "ERROR | Failed to decode key for '{}': {}",
                                platform, err_key
                            );
                        }
                    }
                } else {
                    eprintln!(
                        "PASS-MAN | Failed to retrieve user encoding data for platform '{}'.",
                        platform
                    );
                }
            }
						println!(
							"│ {:<20} │ {:<25} │",
							"PASS-MAN",
							"Your password manager"
					);
            // Replace the last row separator with the table footer
            println!("{}", "└──────────────────────┴───────────────────────────┘");
        }
        Err(err) => {
            eprintln!("ERROR | Failed to retrieve passwords: {}", err);
        }
    }
}
