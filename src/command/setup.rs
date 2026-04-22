use std::io::{self, BufRead, Write};

use crate::cli::setup::SetupArgs;
use crate::config::facility::{load_facility, supported_facilities};
use crate::error::HpcrError;

const PATTERNS: &[(&str, &str)] = &[
    ("perlmutter", "perlmutter"),
    ("frontier", "frontier"),
];

fn detect_facility() -> Option<&'static str> {
    let hostname = std::fs::read_to_string("/etc/hostname")
        .ok()
        .or_else(|| std::env::var("HOSTNAME").ok())?;
    let hostname = hostname.trim().to_lowercase();
    PATTERNS
        .iter()
        .find(|(_, pattern)| hostname.contains(pattern))
        .map(|(name, _)| *name)
}

fn prompt(msg: &str) -> String {
    print!("{msg}");
    io::stdout().flush().unwrap();
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();
    line.trim().to_owned()
}

fn write_config(facility: &str) -> Result<(), HpcrError> {
    let dir = dirs::config_dir()
        .ok_or(HpcrError::LocalConfigNotFound)?
        .join("hpcr");
    std::fs::create_dir_all(&dir).map_err(HpcrError::LocalConfigRead)?;
    let path = dir.join("local.toml");
    if path.exists() {
        let answer = prompt(&format!(
            "Config already exists at {}. Overwrite? [y/N] ",
            path.display()
        ));
        if !matches!(answer.to_lowercase().as_str(), "y" | "yes") {
            println!("Aborted.");
            std::process::exit(0);
        }
    }
    std::fs::write(&path, format!("facility = \"{facility}\"\n"))
        .map_err(HpcrError::LocalConfigRead)?;
    println!("Written: {}", path.display());
    Ok(())
}

fn select_facility() -> String {
    let facilities = supported_facilities();
    println!("Supported facilities: {}", facilities.join(", "));
    loop {
        let answer = prompt("Enter facility name: ");
        if load_facility(&answer).is_ok() {
            return answer;
        }
        println!(
            "Unknown facility '{answer}'. Choose from: {}",
            facilities.join(", ")
        );
    }
}

pub fn run_setup(args: &SetupArgs) -> Result<(), HpcrError> {
    let facility = if let Some(name) = &args.facility {
        load_facility(name)?;
        name.clone()
    } else if let Some(detected) = detect_facility() {
        println!("Detected facility: {detected}");
        let answer = prompt("Use this facility? [Y/n] ");
        if matches!(answer.to_lowercase().as_str(), "" | "y" | "yes") {
            detected.to_owned()
        } else {
            select_facility()
        }
    } else {
        println!("Could not detect facility from hostname.");
        select_facility()
    };

    write_config(&facility)
}
