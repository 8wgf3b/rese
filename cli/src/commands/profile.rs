use clap::Subcommand;
use manticore::profile::Profile;
use std::{
    path::{Path, PathBuf},
    println,
    process::ExitCode,
};

use crate::commands::profile::ProfileAction::Validate;

#[derive(Subcommand)]
pub enum ProfileAction {
    Validate { path: PathBuf },
}

impl ProfileAction {
    pub fn run(self) -> ExitCode {
        match self {
            Validate { path } => validate(&path),
        }
    }
}

fn validate(path: impl AsRef<Path>) -> ExitCode {
    match Profile::load(path) {
        Ok(p) => {
            println!("{p:?}");
            ExitCode::SUCCESS
        }
        Err(manticore::profile::CoreError::Validation(v)) => {
            for e in v {
                println!("{e}")
            }
            ExitCode::FAILURE
        }
        Err(e) => {
            println!("{e}");
            ExitCode::FAILURE
        }
    }
}
