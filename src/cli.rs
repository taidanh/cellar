use clap::Subcommand;

#[derive(Subcommand)]
pub enum CellarCommand {
    /// Adds a new executable
    ///
    /// Arguments:
    ///     <exe> - The path to the executable to add
    Add {
        executable: String,
    },

    /// Gives an executable a new name
    ///
    /// Arguments:
    ///     <from> - The old name for the executable
    ///     <to> - The new name for the executable
    Rename {
        from: String,
        to: String,
    },

    /// Runs a specified command
    ///
    /// Arguments:
    ///     <command_name> - Name of the command to run
    Run {
        executable: String,
    },

    /// Removes an existing command
    ///
    /// Arguments:
    ///     <command_name> - Name of the command to remove
    Remove {
        executable: String,
    },

    /// Initializes the configuration directory
    Init {},
}
