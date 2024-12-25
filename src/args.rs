use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    #[arg(long, hide = true)]
    markdown_help: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a task
    Add { 
        #[arg(num_args=1..)]
        task: Option<Vec<String>> 
    },

    /// Mark a task as completed
    Done {
        #[arg(num_args=1..)]
        id: Option<Vec<usize>>
    },

    /// Mark a task as urgent
    Urge {
        #[arg(num_args=1..)]
        id: Option<Vec<usize>>
    },

    /// Mark a task as normal
    Norm {
        #[arg(num_args=1..)]
        id: Option<Vec<usize>>
    },

    /// Delete an item from the list (with ID)
    Remove {
        #[arg(num_args=1..)]
        id: Option<Vec<usize>>
    },

    /// Make a task seem more important
    Up {
        id: Option<usize>,
        count: Option<usize>
    },

    /// Make a task seem less important
    Down {
        id: Option<usize>,
        count: Option<usize>,
    }
    
}

impl Cli {
    pub fn parse_args() -> Self {
        let cli = Cli::parse();
        if cli.markdown_help {
            clap_markdown::print_help_markdown::<Cli>();
        }
        cli
    }
}