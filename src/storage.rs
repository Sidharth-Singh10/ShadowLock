use std::collections::HashMap;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

pub fn add_site_entry(site_name: &str) -> Result<(), Box<dyn Error>> {
    let sites_file = get_sites_file_path();

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(sites_file)?;

    writeln!(file, "{}", site_name)?;

    Ok(())
}

pub fn load_sites() -> Result<HashMap<String, ()>, Box<dyn Error>> {
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

pub fn get_sites_file_path() -> String {
    let home_dir = dirs::home_dir().expect("Could not find home directory");
    let config_dir = home_dir.join(".password_manager");

    // Create the directory if it doesn't exist
    std::fs::create_dir_all(&config_dir).expect("Failed to create config directory");

    config_dir.join("sites.txt").to_string_lossy().into_owned()
}
