// waker-cli
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

//! # waker-cli
//!
//! A command-line application for sending Wake-on-LAN (WoL) magic packets over the network. It can be used to wake up a machine by its MAC address or by its alias, you can add machines through the CLI and save them to a config file, that way you don't have to memorize their MAC addresses.
//!
//! ## Installation
//!
//! You can install the application from:
//!
//! * Manually (after cloning the repo locally): `cargo install --path .`
//!
//! ## Usage
//!
//! ```bash
//! $ wake --help
//! Wake-On-LAN command line interface for Rust
//!
//! Usage: wake [OPTIONS] [NAME] [COMMAND]
//!
//! Commands:
//!   add   Add machine to the config file
//!   help  Print this message or the help of the given subcommand(s)
//!
//! Arguments:
//!   [NAME]  Name of the machine to wake up, if the `-n` option is specified then this is the MAC address to send the magic packet to (must be in format `xx:xx:xx:xx:xx:xx`)
//!
//! Options:
//!   -n, --name-as-mac              This tells the CLI to use the name as the MAC address to send the magic packet to
//!   -b, --bcast-addr <BCAST_ADDR>  The broadcast address to send the magic packet to (must be `IP:PORT` format) [default: 255.255.255.255:9]  
//!   -B, --bind-addr <BIND_ADDR>    The address to bind the UDP socket to (must be `IP:PORT` format) [default: 0.0.0.0:0]
//!   -h, --help                     Print help
//!   -V, --version                  Print version
//! ```
//!
//! ## Benchmarks
//!
//! The CLI was benchmarked using [Hyperfine](https://github.com/sharkdp/hyperfine). The profiles used were:
//!
//! * Release
//!
//! ```toml
//! [profile.release]
//! lto = true
//! codegen-units = 1
//! opt-level = 3
//! strip = true
//! ```
//!
//! * Fast
//!
//! ```toml
//! [profile.fast]
//! inherits = "release"
//! lto = false
//! ```
//!
//! ### Windows
//!
//! * AMD64, 32GB RAM, Ryzen 7 3800X, Windows 10
//!
//! | Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
//! | ------- | --------- | -------- | -------- | -------- |
//! | `wol 01:23:45:67:89:AB` ([swsnr/wol](https://crates.io/crates/wol) v0.3.1 + release) | 10.3 ± 0.7 | 9.1 | 13.0 | 1.00 |
//! | `wol-rs 01:23:45:67:89:AB` ([fengyc/wol-rs](https://crates.io/crates/wol-rs) v1.1.0 + release) | 10.8 ± 1.5 | 9.5 | 27.9 | 1.05 ± 0.16 |
//! | `wake -n 01:23:45:67:89:AB` (v0.1.0 + fast) | 16.3 ± 0.9 | 14.7 | 21.0 | 1.57 ± 0.13 |
//! | `wake -n 01:23:45:67:89:AB` (v0.1.0 + release) | 16.3 ± 1.3 | 14.5 | 22.9 | 1.58 ± 0.16 |
//!
//! ### Linux
//!
//! * ARM64, 1GB RAM, Orange Pi Zero2, Debian 12
//!
//! | Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
//! | ------- | --------- | -------- | -------- | -------- |
//! | `wol 01:23:45:67:89:AB` ([swsnr/wol](https://crates.io/crates/wol) v0.3.1 + release) | 2.3 ± 0.2 | 2.0 | 3.5 | 1.00 |
//! | `wake -n 01:23:45:67:89:AB` (v0.1.0 + release) | 2.6 ± 0.2 | 2.2 | 3.3 | 1.12 ± 0.12 |
//! | `wake -n 01:23:45:67:89:AB` (v0.1.0 + fast) | 3.0 ± 0.2 | 2.6 | 4.5 | 1.30 ± 0.14 |
//! | `wol-rs 01:23:45:67:89:AB` ([fengyc/wol-rs](https://crates.io/crates/wol-rs) v1.1.0 + release) | 3.5 ± 0.2 | 3.2 | 4.2 | 1.55 ± 0.15 |
//! | `wakeonlan 01:23:45:67:89:AB` ([jpoliv/wakeonlan](https://github.com/jpoliv/wakeonlan) v0.41-12.1) | 92.2 ± 6.0 | 89.9 | 124.2 | 40.31 ± 4.26 |
//!
//! ## MSRV
//!
//! The Minimum Supported Rust Version (MSRV) for `waker-cli` is **1.80**.
//!
//! ## License
//!
//! This project is licensed under the [GNU General Public License v3.0](https://www.gnu.org/licenses/gpl-3.0.en.html).

#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![allow(clippy::doc_markdown)]

mod types;
mod utils;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::Colorize;
use configura::{load_config, Config};
use handy::pattern::{is_close_to_upper_bound, string_similarity};
use inquire::{Confirm, InquireError, Select, Text};
use std::{process::exit, str::FromStr};
use types::{Data, Machine};
use utils::{format_machine_details, validate_mac, validate_text};
use waker::{create_magic_packet, wake_device, Mac, WakeOptions};

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

    #[arg(
        short,
        long,
        help = "The broadcast address to send the magic packet to (must be `IP:PORT` format)",
        default_value = "255.255.255.255:9"
    )]
    bcast_addr: String,

    #[arg(
        short = 'B',
        long,
        help = "The address to bind the UDP socket to (must be `IP:PORT` format)",
        default_value = "0.0.0.0:0"
    )]
    bind_addr: String,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[command(about = "Add machine to the config file", alias = "a")]
    Add,
    // TODO add Edit command
    // TODO add List command
    // TODO add Remove command
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
                    mac: Mac::from_str(&name).context("Invalid MAC address")?,
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

            wake_machine(&machine, &args.bcast_addr, &args.bind_addr)
                .context("Failed to wake machine")?;
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
                    Ok(mach) => wake_machine(&mach, &args.bcast_addr, &args.bind_addr)
                        .context("Failed to wake machine")?,
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

        let default_mac = existing.map_or(String::new(), |m| m.mac.to_string());
        let mac = match Text::new("MAC address:")
            .with_initial_value(&default_mac)
            .with_validator(validate_mac)
            .prompt()
        {
            Ok(m) => m,
            Err(InquireError::OperationInterrupted | InquireError::OperationCanceled) => {
                return Ok(None);
            }
            Err(e) => return Err(e.into()),
        };

        Ok(Some(Machine {
            name,
            mac: Mac::from_str(&mac).context("Invalid MAC address")?,
        }))
    }
}

fn wake_machine(machine: &Machine, bcast_addr: &str, bind_addr: &str) -> Result<()> {
    println!(
        "Waking up machine{} with MAC address {}...",
        if machine.name.is_empty() {
            String::new()
        } else {
            format!(" {}", machine.name.green())
        },
        format!("{:X}", machine.mac).cyan()
    );

    let packet = create_magic_packet(machine.mac)?;

    wake_device(
        WakeOptions::new(&packet)
            .broadcast_address(bcast_addr)
            .bind_address(bind_addr),
    )
    .context("Failed to wake device")?;
    Ok(())
}
