mod cli;
mod commands;
mod password;
mod storage;

use cli::{Cli, Commands};
use clap::Parser;
use dotenv::dotenv;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    
    // Parse command line arguments
    let cli = Cli::parse();
    
    // Default to interactive mode if no command is provided
    match cli.command {
        Some(Commands::Add { site }) => commands::add_site(site),
        Some(Commands::Generate { site }) => commands::generate_password(site),
        Some(Commands::List) => commands::list_sites(),
        None => commands::interactive_mode(),
    }
}