use std::os::unix::fs::symlink;
use std::{env, fs, process::Command};

use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum CellarCommand {
    /// Adds a new executable from some path
    ///
    /// Arguments:
    ///     <executable> - The path to the executable to add
    Add { executable: String },

    /// Gives an executable a new name
    ///
    /// Arguments:
    ///     <from> - The old name for the executable
    ///     <to> - The new name for the executable
    Rename { from: String, to: String },

    /// Runs a specified command
    ///
    /// Arguments:
    ///     <executable> - Name of the executable to run
    Run { executable: String },

    /// Removes an existing command
    ///
    /// Arguments:
    ///     <executable> - Name of the executable to remove
    Remove { executable: String },

    /// Initializes the configuration directory
    Init {},
}

#[derive(Parser)]
#[command(name = "Cellar", about, author, version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: CellarCommand,

    #[arg(short, long, default_value_t = format!(""))]
    pub path: String,

    #[arg(short, long, default_value_t = false)]
    pub copy: bool,
}

impl CellarCommand {
    pub fn get_match(&self, copy: bool) {
        use CellarCommand::*;

        match self {
            Add { executable } => {
                let abs_exe = fs::canonicalize(executable)
                    .expect("Unable to get absolute path to executable");
                let executable = get_exe_name(executable);
                let to = cellar_config(format!("scripts/{}", executable));
                let _ = fs::remove_file(&to);
                match copy {
                    true => {
                        let _ = fs::copy(abs_exe, to).expect("Error copying executable");
                    }
                    false => {
                        println!("exe: {:?}", abs_exe);
                        println!("to: {}", to);
                        let _ = symlink(abs_exe, to).expect("Error symlinking executable");
                    }
                };
            }
            Rename { from, to } => {
                let from = cellar_config(format!("scripts/{}", from));
                let to = cellar_config(format!("scripts/{}", to));
                fs::rename(from.clone(), to).expect(&format!("Error: {} does not exist.", from));
            }
            Run { executable } => {
                let output = Command::new(cellar_config(format!("scripts/{}", executable)))
                    .output()
                    .expect("Failed to execute command");
                println!(
                    "output: {}",
                    String::from_utf8(output.stdout).expect("Failed to read output")
                );
            }
            Remove { executable } => {
                let _ = fs::remove_file(cellar_config(format!("scripts/{}", executable)));
            }
            Init {} => {
                fs::create_dir_all(cellar_config("scripts"))
                    .expect("Unable to create ~/.config/cellar/scripts/");
                println!("Created ~/.config/cellar/scripts/");
            }
        }
    }
}

fn cellar_config<S: ToString>(dir: S) -> String {
    let home = env::var("HOME").expect("Unable to read $HOME");
    format!("{}/.config/cellar/{}", home, dir.to_string())
}

fn get_exe_name<S: ToString>(dir: S) -> String {
    dir.to_string()
        .split('/')
        .last()
        .unwrap_or(&dir.to_string())
        .to_string()
}
