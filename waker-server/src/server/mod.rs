mod utils;

use anyhow::{Context, Result};
use chrono::Local;
use log::{debug, error, LevelFilter};
use rayon::ThreadPoolBuilder;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};
use std::{
    fs::{create_dir_all, OpenOptions},
    path::{Path, PathBuf},
    sync::Arc,
};
use utils::{get_num_threads, is_env};

const ENV_DEBUG: &str = "DEBUG";

const FOLDER_DATA: &str = "data";
const FOLDER_LOGS: &str = "logs";

#[derive(Debug)]
pub struct Service {
    pub debug: bool,
    data_folder: PathBuf,
    logs_folder: PathBuf,
}

impl Service {
    fn init_logger(&self) -> Result<()> {
        let filter = if self.debug {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        };
        let logs_folder = self.logs_folder();

        create_dir_all(logs_folder).context("Failed to create logs folder")?;

        let filename = Local::now().format("%Y-%m-%d").to_string();
        let log_file_path = logs_folder.join(format!("{filename}.log"));
        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file_path)
            .context("Failed to open or create log file")?;

        CombinedLogger::init(vec![
            TermLogger::new(
                filter,
                Config::default(),
                TerminalMode::Mixed,
                ColorChoice::Auto,
            ),
            WriteLogger::new(filter, Config::default(), log_file),
        ])
        .context("Failed to initialize logger")?;

        Ok(())
    }

    pub fn data_folder(&self) -> &Path {
        &self.data_folder
    }

    pub fn logs_folder(&self) -> &Path {
        &self.logs_folder
    }
}

pub async fn start(
    host: &str,
    port: u16,
    debug: bool,
    data_folder_opt: Option<PathBuf>,
    threads_opt: Option<usize>,
) -> Result<()> {
    let debug = is_env(ENV_DEBUG) || debug;
    let data_folder = data_folder_opt.unwrap_or_else(|| PathBuf::from(FOLDER_DATA));
    let service = Service {
        debug,
        data_folder: data_folder.clone(),
        logs_folder: data_folder.join(FOLDER_LOGS),
    };

    service
        .init_logger()
        .context("Failed to initialize logger")?;

    let num_threads = get_num_threads(threads_opt);
    ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .context("Failed to create thread pool")?;
    debug!("t={num_threads}");

    if let Err(e) = proceed(Arc::new(service), host, port)
        .await
        .context("Failed to proceed with serving backend")
    {
        error!("Error: {e:?}");
        eprintln!("Error: {e:?}");
    }

    Ok(())
}

async fn proceed(service: Arc<Service>, host: &str, port: u16) -> Result<()> {
    debug!("service={service:#?}");

    Ok(())
}
