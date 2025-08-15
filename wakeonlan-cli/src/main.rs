#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

mod types;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::Colorize;
use configura::{load_config, Config};
use handy::pattern::{is_close_to_upper_bound, string_similarity};
use inquire::{validator::Validation, Confirm, InquireError, Select, Text};
use std::{process::exit, str::FromStr};
use types::{Data, Machine};
use wakeonlan::Mac;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct App {
    #[arg(
        help = "Name of the machine to wake up, if the `-n` option is specified then this is the MAC address to send the magic packet to (must be in format `xx:xx:xx:xx:xx:xx`)"
    )]
    name: Option<String>,

    #[arg(
        short = 'n',
        long,
        help = "This tells the CLI to use the name as the MAC address to send the magic packet to"
    )]
    name_as_mac: bool,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[command(about = "Add machine to the config file", alias = "a")]
    Add,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", format!("{e:?}").red());
        exit(1);
    }
}

fn run() -> Result<()> {
    let args = App::parse();
    let mut config: Data = load_config().context("Failed to load config file")?;

    match args.name {
        Some(name) => {
            let machine = if args.name_as_mac {
                Machine {
                    name: String::new(),
                    mac: name,
                }
            } else {
                if config.machines.is_empty() {
                    println!("No machines found in config file");
                    return Ok(());
                }

                if let Some(mach) = config.find_best_machine(&name) {
                    mach
                } else {
                    println!("No machine found with name: {name}");
                    return Ok(());
                }
            };

            wake_machine(&machine).context("Failed to wake machine")?;
        }

        None => match args.command {
            Some(Command::Add) => config.add_machine().context("Failed to add machine")?,

            None => {
                if config.machines.is_empty() {
                    println!("No machines found in config file");
                    return Ok(());
                }

                let machines = config.machines;
                match Select::new("Choose a machine to wake up:", machines).prompt() {
                    Ok(mach) => wake_machine(&mach).context("Failed to wake machine")?,
                    Err(InquireError::OperationInterrupted | InquireError::OperationCanceled) => {
                        return Ok(())
                    }
                    Err(e) => return Err(e.into()),
                }
            }
        },
    }

    Ok(())
}

impl Data {
    fn add_machine(&mut self) -> Result<()> {
        let Some(machine) = self
            .prompt_machine(None)
            .context("Failed to prompt a machine")?
        else {
            return Ok(());
        };

        if Confirm::new("Do you want to save this machine?")
            .with_default(false)
            .with_help_message(&format_machine_details(&machine))
            .prompt()?
        {
            self.machines.push(machine);
            self.save().context("Failed to save config file")?;

            println!("{}", "Machine added successfully".green());
        }

        Ok(())
    }

    fn find_best_machine(&self, name: &str) -> Option<Machine> {
        let mut best_score = 0.0;
        let mut best_match = None;

        for machine in &self.machines {
            let score = string_similarity(&machine.name, name);

            if score > best_score {
                best_score = score;
                best_match = Some(machine);
            }

            if is_close_to_upper_bound(score) {
                break;
            }
        }

        best_match.cloned()
    }

    fn prompt_machine(&self, existing: Option<&Machine>) -> Result<Option<Machine>> {
        let default_name = existing.map_or("", |m| &m.name);
        let name = match Text::new("Machine name:")
            .with_initial_value(default_name)
            .with_validator(validate_text)
            .prompt()
        {
            Ok(n) => n,
            Err(InquireError::OperationInterrupted | InquireError::OperationCanceled) => {
                return Ok(None);
            }
            Err(e) => return Err(e.into()),
        };

        if self
            .machines
            .iter()
            .any(|m| string_similarity(&m.name, &name) > 0.9)
        {
            println!("Machine already exists: {name}");
            return Ok(None);
        }

        let default_mac = existing.map_or("", |m| &m.mac);
        let mac = match Text::new("MAC address:")
            .with_initial_value(default_mac)
            .with_validator(validate_mac)
            .prompt()
        {
            Ok(m) => m,
            Err(InquireError::OperationInterrupted | InquireError::OperationCanceled) => {
                return Ok(None);
            }
            Err(e) => return Err(e.into()),
        };

        Ok(Some(Machine { name, mac }))
    }
}

fn format_machine_details(machine: &Machine) -> String {
    format!(
        "\nName: {}\nMAC: {}\n",
        machine.name.green(),
        machine.mac.cyan()
    )
}

#[allow(clippy::unnecessary_wraps)]
fn validate_mac(input: &str) -> Result<Validation, Box<dyn std::error::Error + Send + Sync>> {
    match Mac::from_str(input) {
        Ok(_) => Ok(Validation::Valid),
        Err(e) => Ok(Validation::Invalid(e.to_string().into())),
    }
}

#[allow(clippy::unnecessary_wraps)]
fn validate_text(input: &str) -> Result<Validation, Box<dyn std::error::Error + Send + Sync>> {
    if input.trim().is_empty() {
        Ok(Validation::Invalid("Field cannot be empty".into()))
    } else {
        Ok(Validation::Valid)
    }
}

fn wake_machine(machine: &Machine) -> Result<()> {
    Mac::from_str(&machine.mac)?;

    println!(
        "Waking up machine{} with MAC address {}...",
        if machine.name.is_empty() {
            String::new()
        } else {
            format!(" {}", machine.name.green())
        },
        machine.mac.cyan()
    );
    Ok(())
}
