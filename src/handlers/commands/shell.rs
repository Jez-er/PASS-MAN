use std::process::{Command, exit};

pub fn activate() {
    let status = Command::new("bash")
        .arg("-c")
        .arg("echo 'PASS-MAN | Welcome to your Password Manager Console!' && bash")
        .status()
        .expect("PASS-MAN |Failed to start shell");

    if !status.success() {
        eprintln!("PASS-MAN | Shell exited with error status: {}", status);
        exit(1);
    }
}
