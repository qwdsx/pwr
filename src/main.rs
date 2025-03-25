
use cli_clipboard;

mod cli;
use cli::*;

mod gen;
use gen::*;

use dotenv::*;

fn main() {
    let cli = Cli::new();

    match cli.command {
        Commands::Gen(args) => {

            let length = match args.length {
                Some(s) => s,
                None => 16
            };

            let numbers = !args.numbers;
            let uppercase = !args.uppercase;
            let lowercase = !args.lowercase;
            let symbols = !args.symbols;

            let config = Config {
                length,
                numbers,
                uppercase,
                lowercase,
                symbols
            };

            let password = gen::generate_password(&config);
            println!("Generated password: {}", password);
        }
    }
}