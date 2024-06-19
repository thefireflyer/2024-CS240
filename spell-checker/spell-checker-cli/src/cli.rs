use std::path::PathBuf;

use clap::{Parser, Subcommand};

///////////////////////////////////////////////////////////////////////////////

#[derive(Parser)]
#[command(
    author="Aidan Beil",
    about="Spell checker", 
    long_about = Some("CS240 Midterm | Spell checker CLI")
)]
pub struct Config {
    /// Path to dictionary
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Command,
}

//---------------------------------------------------------------------------//

#[derive(Subcommand)]
pub enum Command {
    Check {
        text: Vec<String>,
    },
    Suggest {
        word: String,
        /// Max number of spelling suggestion
        #[arg(short, long, default_value_t = 10)]
        limit: usize,
    },
    CheckSuggest {
        text: Vec<String>,
        /// Max number of spelling suggestion
        #[arg(short, long, default_value_t = 10)]
        limit: usize,
    },
    Add {
        word: String,
    },
}

///////////////////////////////////////////////////////////////////////////////
