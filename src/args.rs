use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author="Adam Subora", version, about="A minimal cli to-do app", long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    #[arg(long, hide=true)]
    markdown_help: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a task
    #[command(alias="a")]
    Add {
        #[arg(num_args=1..)]
        task: Option<Vec<String>>,
    },
    /// Mark a task as in progress
    #[command(alias="ip", alias="p")]
    Prog {
        #[arg(num_args=1..)]
        id: Option<Vec<usize>>
    },
    /// Mark a task as completed
    #[command(alias="d")]
    Done {
        #[arg(num_args=1..)]
        id: Option<Vec<usize>>,
    },
    /// Mark a task as urgent
    #[command(alias="u")]
    Urge {
        #[arg(num_args=1..)]
        id: Option<Vec<usize>>,
    },
    /// Mark a task as normal
    #[command(alias="n")]
    Norm {
        #[arg(num_args=1..)]
        id: Option<Vec<usize>>,
    },
    /// Delete an item from the list
    #[command(alias="rm", alias="r")]
    Remove {
        #[arg(num_args=1..)]
        id: Option<Vec<String>>,
    },
    // /// Make a task seem more important
    // Up {
    //     id: Option<usize>,
    //     count: Option<usize>,
    // },
    // /// Make a task seem less important
    // Down {
    //     id: Option<usize>,
    //     count: Option<usize>,
    // },
    /// Reset task IDs with current positions
    Reset,
}

impl Cli {
    pub fn parse_args() -> Self {
        let cli = Cli::parse();
        cli
    }
}
