use std::{process::Command, env, fs, path::Path};

use clap::Parser;
use cli::*;

mod cli;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { exe } => {
            println!("Adding {}", exe);
            let home = env::var("HOME").expect("Unable to read $HOME");
            let command_name = Path::new(&exe)
                .file_name().expect("Unable to access executable")
                .to_str().expect("Unable to read executable name").to_string();
            let output = fs::copy(exe, format!("{}/.config/cellar/scripts/{}", home, command_name));
        },
        Commands::Run { command_name } => {
            let home = env::var("HOME").expect("Unable to read $HOME");
            let output = Command::new(cellar_config(format!("/scripts/{}", command_name)))
                .output()
                .expect("Failed to execute command");
            println!("output: {}", String::from_utf8(output.stdout).expect("Failed to read output"));
        },
        Commands::Remove { command_name } => {
            let _ = fs::remove_file(cellar_config(format!("/scripts/{}", command_name)));
        },
        Commands::Init {  } => {
            fs::create_dir_all(format!("{}/scripts", cellar_config("scripts")))
            .expect("Unable to create ~/.config/cellar/scripts/");
            println!("Created ~/.config/cellar/scripts/");
        }
    }
}

fn cellar_config<S>(dir: S) -> String
where
    S: ToString,
{
    let home = env::var("HOME").expect("Unable to read $HOME");
    format!("{}/.config/cellar/", home)
}

#[derive(Parser)]
#[command(name = "Cellar", about, author, version)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(default_value_t = format!(""))]
    path: String,
}
