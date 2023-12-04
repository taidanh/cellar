use cli::*;

mod cli;

fn main() {
    let cli = Cli::parse();

    println!("Hello, world!");
}
