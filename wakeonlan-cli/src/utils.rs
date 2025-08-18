// wakeonlan-cli
// Copyright (C) 2025 DarkCeptor44
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

use crate::types::Machine;
use colored::Colorize;
use inquire::validator::Validation;
use std::str::FromStr;
use wakeonlan::Mac;

pub fn format_machine_details(machine: &Machine) -> String {
    format!(
        "\nName: {}\nMAC: {}\n",
        machine.name.green(),
        machine.mac.to_string().cyan()
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
