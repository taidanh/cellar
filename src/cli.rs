use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
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
