pub mod commands_mod {
    use clap::{Parser, Subcommand};

    #[derive(Subcommand)]
    pub enum Commands {
        /// Create a initial config
        Init {
            /// Template for current project
            #[arg(short, long)]
            template: String,
        },
    }
}
