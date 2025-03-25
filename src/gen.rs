
use rand::*;
use rand::distributions::Uniform;

pub struct Config {
    pub length: u16,
    pub numbers: bool,
    pub uppercase: bool,
    pub lowercase: bool,
    pub symbols: bool
}

pub fn generate_password(config: &Config) -> String {
    let mut password = String::new();
    let mut valid_chars = Vec::<char>::new();

    let numbers: String = "1234567890".into();
    let uppercase: String = "ABCDEFGHIJKLMNOPQRSTUVWYZ".into();
    let lowercase: String = "abcdefghijklmnopqrstuvwyz".into();
    let symbols: String = "@#$%&=+?".into();

    if config.uppercase {
        for i in uppercase.chars() {
            valid_chars.push(i);
        }
    }

    if config.lowercase {
        for i in lowercase.chars() {
            valid_chars.push(i);
        }
    }
    
    if config.symbols {
        for i in symbols.chars() {
            valid_chars.push(i);
        }
    }

    if config.numbers {
        for i in numbers.chars() {
            valid_chars.push(i);
        }
    }

    let mut rng = rand::thread_rng();
    let range = Uniform::from(10..valid_chars.len());
    
    for _ in 0..config.length {
        password.push(valid_chars[rng.sample(range)]);
    }

    password
}