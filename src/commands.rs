use crate::password;
use crate::storage;
use dialoguer::{Input, Password, Select};
use std::error::Error;

pub fn interactive_mode() -> Result<(), Box<dyn Error>> {
    println!("Password Manager");
    println!("---------------");

    // Ask for master password
    let master_password = Password::new()
        .with_prompt("Enter your master password")
        .interact()?;

    // Load sites
    let sites = storage::load_sites()?;

    if sites.is_empty() {
        println!("No sites found. Add a site first.");
        let site_name = Input::<String>::new()
            .with_prompt("Enter site name to add")
            .interact()?;

        storage::add_site_entry(&site_name)?;
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
    let real_password = password::generate_password(&master_password, site_name, &pre_password)?;
    println!("Your password for '{}': {}", site_name, real_password);

    Ok(())
}

pub fn add_site(site_option: Option<String>) -> Result<(), Box<dyn Error>> {
    let site_name = match site_option {
        Some(site) => site,
        None => Input::<String>::new()
            .with_prompt("Enter site name")
            .interact()?,
    };

    storage::add_site_entry(&site_name)?;
    println!("Site '{}' added successfully", site_name);

    Ok(())
}

pub fn generate_password(site_option: Option<String>) -> Result<(), Box<dyn Error>> {
    let sites = storage::load_sites()?;

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
    let real_password = password::generate_password(&master_password, &site_name, &pre_password)?;
    println!("Your password for '{}': {}", site_name, real_password);

    Ok(())
}

pub fn list_sites() -> Result<(), Box<dyn Error>> {
    let sites = storage::load_sites()?;

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
