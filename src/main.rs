use std::io::{self, Write};
use std::thread;
use std::time::Duration;

// Function to obfuscate characters
fn obfuscate(c: char) -> char {
    (c as u8 ^ 0x5A) as char
}

// Function to get the obfuscated correct password
fn get_obfuscated_password() -> String {
    let password = "SecurePass123";
    password.chars().map(obfuscate).collect()
}

// Function to check the password
fn check_password(input: &str) -> bool {
    let obfuscated_password = get_obfuscated_password();
    let obfuscated_input: String = input.chars().map(obfuscate).collect();
    obfuscated_input == obfuscated_password
}

fn main() {
    println!("Welcome to RustCrackMaster!");
    print!("Please enter the activation password: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        let input = input.trim_end(); // Remove trailing newline

        // Small delay to complicate analysis
        thread::sleep(Duration::from_millis(500));

        if check_password(input) {
            println!("Congratulations! You have cracked RustCrackMaster.");
        } else {
            println!("Incorrect password. Please try again.");
        }
    } else {
        println!("Error reading input.");
    }
}
