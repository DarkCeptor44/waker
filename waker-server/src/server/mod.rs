mod utils;

use anyhow::Result;
use std::path::PathBuf;
use utils::is_env;

const ENV_DEBUG: &str = "DEBUG";

pub async fn start(
    host: &str,
    port: u16,
    debug: bool,
    data_folder_opt: Option<PathBuf>,
    threads_opt: Option<usize>,
) -> Result<()> {
    let debug = is_env(ENV_DEBUG) || debug;

    Ok(())
}
