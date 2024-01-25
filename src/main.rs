use clap::Parser;
use cli::*;

mod cli;

fn main() {
    let cli = Cli::parse();
    cli.command.get_match(cli.copy);
}
