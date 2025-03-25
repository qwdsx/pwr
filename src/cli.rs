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

    #[arg(short, long, help = "boolean to NOT include numbers")]
    pub numbers: bool,

    #[arg(short, long, help = "boolean to NOT include uppercase letters")]
    pub uppercase: bool,

    #[arg(short, long, help = "boolean to NOT include lowercase letters")]
    pub lowercase: bool,

    #[arg(short, long, help = "boolean to NOT include symbols (@#$%&=+?)")]
    pub symbols: bool,
}

impl Cli {
    pub fn new() -> Self {
        Self::parse()
    }
}