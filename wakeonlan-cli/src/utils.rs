use crate::types::Machine;
use colored::Colorize;
use inquire::validator::Validation;
use std::str::FromStr;
use wakeonlan::Mac;

pub fn format_machine_details(machine: &Machine) -> String {
    format!(
        "\nName: {}\nMAC: {}\n",
        machine.name.green(),
        machine.mac.cyan()
    )
}

#[allow(clippy::unnecessary_wraps)]
pub fn validate_mac(input: &str) -> Result<Validation, Box<dyn std::error::Error + Send + Sync>> {
    match Mac::from_str(input) {
        Ok(_) => Ok(Validation::Valid),
        Err(e) => Ok(Validation::Invalid(e.to_string().into())),
    }
}

#[allow(clippy::unnecessary_wraps)]
pub fn validate_text(input: &str) -> Result<Validation, Box<dyn std::error::Error + Send + Sync>> {
    if input.trim().is_empty() {
        Ok(Validation::Invalid("Field cannot be empty".into()))
    } else {
        Ok(Validation::Valid)
    }
}
