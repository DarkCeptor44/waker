use anyhow::Result;
use std::path::PathBuf;

pub async fn start(
    host: &str,
    port: u16,
    debug: bool,
    data_folder_opt: Option<PathBuf>,
    threads_opt: Option<usize>,
) -> Result<()> {
    Ok(())
}
