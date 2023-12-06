use clap::Subcommand;

#[derive(Subcommand)]
pub enum CellarCommand {
    /// Adds a new executable
    ///
    /// Arguments:
    ///     <exe> - The path to the executable to add
    Add {
        #[arg(short, long, help = "Path to the executable to add")]
        exe: String,
    },

    /// Runs a specified command
    ///
    /// Arguments:
    ///     <command_name> - Name of the command to run
    Run {
        #[arg(short, long, help = "Name of the command to run")]
        command_name: String,
    },

    /// Removes an existing command
    ///
    /// Arguments:
    ///     <command_name> - Name of the command to remove
    Remove {
        #[arg(short, long, help = "Name of the command to remove")]
        command_name: String,
    },

    /// Initializes the configuration directory
    Init {},
}
