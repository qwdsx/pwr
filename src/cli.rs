use clap::{Args, Parser, Subcommand};

const ABOUT: &str= "CLI app that generates random passwords";

#[derive(Parser)]
#[clap(about = ABOUT, version)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate a random password
    Gen(GenArgs)
}

#[derive(Args)]
pub struct GenArgs {
    #[arg(long, help = "length of password (default 16)")]
    pub length: Option<u16>,

    #[arg(short, long, help = "boolean to NOT include numbers in the password")]
    pub numbers: bool,

    #[arg(short, long, help = "boolean to NOT include uppercase letters in the password")]
    pub uppercase: bool,

    #[arg(short, long, help = "boolean to NOT include lowercase letters in the password")]
    pub lowercase: bool,

    #[arg(short, long, help = "boolean to NOT include symbols (@#$%&=+?) in the password")]
    pub symbols: bool,
}

impl Cli {
    pub fn new() -> Self {
        Self::parse()
    }
}