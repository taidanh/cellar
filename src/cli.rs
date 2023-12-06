use clap::Subcommand;

#[derive(Subcommand)]
pub enum CellarCommand {
    Add {
        exe: String,
    },

    Run {
        command_name: String,
    },

    Remove {
        command_name: String,
    },

    Init {},
}
