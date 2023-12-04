use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Cellar", about, author, version)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(default_value_t = format!(""))]
    path: String,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        command_name: String,
    }
}
