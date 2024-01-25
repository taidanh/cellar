use std::{process::Command, env, fs, path::Path};
use std::os::unix::fs::symlink;

use clap::Parser;
use cli::*;

mod cli;

fn main() {
    let cli = Cli::parse();
    let home: String = env::var("HOME").expect("Unable to head $HOME");

    match cli.command {
        CellarCommand::Add { executable } => {
            let command_name = Path::new(&executable)
                .file_name().expect("Unable to access executable")
                .to_str().expect("Unable to read executable name").to_string();
            let exe = fs::canonicalize(executable).expect("Unable to get absolute path to executable");
            let to = format!("{}/.config/cellar/scripts/{}", home, command_name);
            let _ = fs::remove_file(&to);
            match cli.copy {
                true => {
                    let _ = fs::copy(exe, to).expect("Error copying executable");
                },
                false => {
                    let _ = symlink(exe,to).expect("Error symlinking executable");
                },
            };
        },
        CellarCommand::Rename { from, to} => {
            let from = cellar_config(format!("/scripts/{}", from));
            let to = cellar_config(format!("/scripts/{}", to));
            fs::rename(from.clone(), to)
                .expect(&format!("Error: {} does not exist.", from));
        },
        CellarCommand::Run { executable } => {
            let output = Command::new(cellar_config(format!("/scripts/{}", executable)))
                .output()
                .expect("Failed to execute command");
            println!("output: {}", String::from_utf8(output.stdout).expect("Failed to read output"));
        },
        CellarCommand::Remove { executable } => {
            let _ = fs::remove_file(cellar_config(format!("/scripts/{}", executable)));
        },
        CellarCommand::Init {  } => {
            fs::create_dir_all(cellar_config("scripts"))
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
    format!("{}/.config/cellar/{}", home, dir.to_string())
}

#[derive(Parser)]
#[command(name = "Cellar", about, author, version)]
pub struct Cli {
    #[command(subcommand)]
    command: CellarCommand,

    #[arg(short, long, default_value_t = format!(""))]
    path: String,

    #[arg(short, long, default_value_t = false)]
    copy: bool,
}
