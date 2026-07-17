// backend/src/bin/generate_hash.rs

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if password was provided as argument
    let password = if args.len() > 1 {
        args[1].clone()
    } else {
        // If not, read from stdin (for piping)
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_ok() {
            input.trim().to_string()
        } else {
            // Default fallback
            "admin123".to_string()
        }
    };

    if password.is_empty() {
        eprintln!("Error: Password cannot be empty");
        std::process::exit(1);
    }

    // Generate Argon2 hash
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(hash) => {
            println!("{}", hash.to_string());
        }
        Err(e) => {
            eprintln!("Failed to hash password: {}", e);
            std::process::exit(1);
        }
    }
}
