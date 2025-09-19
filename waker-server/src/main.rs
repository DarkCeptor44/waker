#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

mod server;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::Colorize;
use dotenvy::dotenv;
use std::{path::PathBuf, process::exit};

#[derive(Parser)]
#[command(author, version, about, long_about = None, propagate_version = true)]
struct App {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(about = "Run the server")]
    Start {
        #[arg(
            short = 'H',
            long,
            help = "Host to listen on",
            default_value = "0.0.0.0"
        )]
        host: String,

        #[arg(short, long, help = "Port to listen on", default_value_t = 8080)]
        port: u16,

        #[arg(short = 'D', long, help = "Data folder to use (default: ./data)")]
        data_folder: Option<PathBuf>,

        #[arg(
            short,
            long,
            help = "Number of threads to use (default: 75% of available cores)"
        )]
        threads: Option<usize>,

        #[arg(short, long, help = "Enable debug logging", default_value_t)]
        debug: bool,
    },
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    if let Err(e) = run().await {
        eprintln!("{}", format!("{e:?}").red());
        exit(1);
    }
}

async fn run() -> Result<()> {
    let args = App::parse();

    match args.command {
        Command::Start {
            host,
            port,
            threads,
            debug,
            data_folder,
        } => server::start(&host, port, debug, data_folder, threads)
            .await
            .context("Failed to start backend")?,
    }

    Ok(())
}
