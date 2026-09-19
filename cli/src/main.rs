use clap::{Parser, Subcommand};

use crate::{Commands::Profile, commands::profile::ProfileAction};

pub mod commands;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Profile {
        #[command(subcommand)]
        action: ProfileAction,
    },
}

fn main() {
    //let a = 21;
    let cli = Cli::parse();
    match cli.command {
        Profile { action } => action.run(),
    };
}
