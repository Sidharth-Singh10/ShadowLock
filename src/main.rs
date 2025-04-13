use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString},
};
use base64::{Engine, prelude::BASE64_STANDARD};
use clap::{Parser, Subcommand};
use dialoguer::{Input, Password, Select};
use dotenv::dotenv;
use std::collections::HashMap;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
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

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    // Parse command line arguments
    let cli = Cli::parse();

    // Default to interactive mode if no command is provided
    match cli.command {
        Some(Commands::Add { site }) => add_site(site),
        Some(Commands::Generate { site }) => generate_password(site),
        Some(Commands::List) => list_sites(),
        None => interactive_mode(),
    }
}

fn interactive_mode() -> Result<(), Box<dyn Error>> {
    println!("Password Manager");
    println!("---------------");

    // Ask for master password
    let master_password = Password::new()
        .with_prompt("Enter your master password")
        .interact()?;

    // Load sites
    let sites = load_sites()?;

    if sites.is_empty() {
        println!("No sites found. Add a site first.");
        let site_name = Input::<String>::new()
            .with_prompt("Enter site name to add")
            .interact()?;

        add_site_entry(&site_name)?;
        return Ok(());
    }

    // Show list of sites and let user select one
    let site_names: Vec<&String> = sites.keys().collect();
    let selection = Select::new()
        .with_prompt("Select a site")
        .items(&site_names)
        .interact()?;

    let site_name = site_names[selection];

    // Ask for pre-password
    let pre_password = Password::new()
        .with_prompt("Enter pre-password for this site")
        .interact()?;

    // Generate password
    generate_password_for_site(&master_password, site_name, &pre_password)?;

    Ok(())
}

fn add_site(site_option: Option<String>) -> Result<(), Box<dyn Error>> {
    let site_name = match site_option {
        Some(site) => site,
        None => Input::<String>::new()
            .with_prompt("Enter site name")
            .interact()?,
    };

    add_site_entry(&site_name)?;
    println!("Site '{}' added successfully", site_name);

    Ok(())
}

fn add_site_entry(site_name: &str) -> Result<(), Box<dyn Error>> {
    let sites_file = get_sites_file_path();

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(sites_file)?;

    writeln!(file, "{}", site_name)?;

    Ok(())
}

fn generate_password(site_option: Option<String>) -> Result<(), Box<dyn Error>> {
    let sites = load_sites()?;

    if sites.is_empty() {
        println!("No sites found. Add a site first.");
        return Ok(());
    }

    let site_name = match site_option {
        Some(site) => {
            if !sites.contains_key(&site) {
                println!("Site '{}' not found", site);
                return Ok(());
            }
            site
        }
        None => {
            // Show list of sites and let user select one
            let site_names: Vec<&String> = sites.keys().collect();
            let selection = Select::new()
                .with_prompt("Select a site")
                .items(&site_names)
                .interact()?;

            site_names[selection].clone()
        }
    };

    // Ask for master password
    let master_password = Password::new()
        .with_prompt("Enter your master password")
        .interact()?;

    // Ask for pre-password
    let pre_password = Password::new()
        .with_prompt("Enter pre-password for this site")
        .interact()?;

    // Generate password
    generate_password_for_site(&master_password, &site_name, &pre_password)?;

    Ok(())
}

fn generate_password_for_site(
    master_password: &str,
    site_name: &str,
    pre_password: &str,
) -> Result<(), Box<dyn Error>> {
    let salt_str = std::env::var("PASSWORD_SALT").expect("PASSWORD_SALT must be set in .env file");

    let argon2 = Argon2::default();
    let salt =
        SaltString::from_b64(BASE64_STANDARD.encode(&salt_str).as_str()).expect("Invalid salt");

    let password_hash = argon2
        .hash_password(master_password.as_bytes(), &salt)
        .expect("Password hashing failed");

    // Extract the hash part to use as our private key
    let hash_string = password_hash.hash.expect("Hash value missing").to_string();

    // Create a BLAKE3 hash of the Argon2 hash to get a 32-byte key
    let keyed_hash = blake3::hash(hash_string.as_bytes());
    let private_key_array = keyed_hash.as_bytes();

    let mut blake_hasher = blake3::Hasher::new_keyed(private_key_array);

    // Add site name
    blake_hasher.update(site_name.as_bytes());

    // Add pre_password
    blake_hasher.update(pre_password.as_bytes());

    let real_password = blake_hasher.finalize().as_bytes().to_vec();

    let real_password = real_password
        .iter()
        .take(15)
        .map(|&byte| (byte % 94 + 33) as char) // Map bytes to printable ASCII range
        .collect::<String>();

    println!("Generated password for '{}': {}", site_name, real_password);

    Ok(())
}

fn list_sites() -> Result<(), Box<dyn Error>> {
    let sites = load_sites()?;

    if sites.is_empty() {
        println!("No sites found");
        return Ok(());
    }

    println!("Available sites:");
    for (i, site) in sites.keys().enumerate() {
        println!("{}. {}", i + 1, site);
    }

    Ok(())
}

fn load_sites() -> Result<HashMap<String, ()>, Box<dyn Error>> {
    let sites_file = get_sites_file_path();

    let mut sites = HashMap::new();

    if Path::new(&sites_file).exists() {
        let file = File::open(sites_file)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let site = line?;
            if !site.trim().is_empty() {
                sites.insert(site, ());
            }
        }
    }

    Ok(sites)
}

fn get_sites_file_path() -> String {
    let home_dir = dirs::home_dir().expect("Could not find home directory");
    let config_dir = home_dir.join(".password_manager");

    // Create the directory if it doesn't exist
    std::fs::create_dir_all(&config_dir).expect("Failed to create config directory");

    config_dir.join("sites.txt").to_string_lossy().into_owned()
}
