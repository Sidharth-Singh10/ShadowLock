use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new site to the database
    Add {
        /// Site name
        #[arg(short, long)]
        site: Option<String>,
    },
    /// Generate a password for an existing site
    Generate {
        /// Site name
        #[arg(short, long)]
        site: Option<String>,
    },
    /// List all available sites
    List,
}