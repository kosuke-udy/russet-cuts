use clap::{Parser, Subcommand, ArgAction};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Optional name to operate on
    pub name: Option<String>,

    /// Turn debugging information on
    #[arg(short='d', long, global=true, action = ArgAction::SetTrue)]
    pub debug: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Actions
#[derive(Subcommand)]
pub enum Commands {
    /// Run shortcut
    Run {
        /// Shortcut name
        name: String,

        #[arg(short='i', long, action=ArgAction::Set)]
        input_path: Option<String>,
    },
}

pub fn load() -> Cli {
    Cli::parse()
}