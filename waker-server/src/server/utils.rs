use log::warn;
use std::{env::var, str::FromStr};

pub fn get_env<S, T>(name: S, default: T) -> T
where
    S: AsRef<str>,
    T: FromStr,
{
    let value = var(name.as_ref()).ok().and_then(|s| s.parse().ok());
    value.unwrap_or(default)
}

#[allow(
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation
)]
pub fn get_num_threads<T>(threads: T) -> usize
where
    T: Into<Option<usize>>,
{
    let cores = num_cpus::get();
    let mut num_threads = (cores as f64 * 0.75).round() as usize;

    if let Ok(env_threads_str) = var("NUM_THREADS") {
        if let Ok(env_threads) = env_threads_str.parse::<usize>() {
            num_threads = env_threads;
        } else {
            warn!("Failed to parse NUM_THREADS env var: {env_threads_str}");
        }
    }

    if let Some(threads) = threads.into() {
        num_threads = threads;
    }

    if num_threads == 0 {
        num_threads = 1;
        warn!("Number of threads was 0, using 1");
    }

    if num_threads > cores {
        num_threads = cores;
        warn!("Number of threads provided ({num_threads}) was greater than the number of cores ({cores}), using the number of cores");
    }

    num_threads
}

pub fn is_env<S>(name: S) -> bool
where
    S: AsRef<str>,
{
    !var(name.as_ref()).unwrap_or_default().is_empty()
}
