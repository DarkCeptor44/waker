use configura::{formats::JsonFormat, Config};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

const CONFIG_NAME: &str = "wol";

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Data {
    pub machines: Vec<Machine>,
}

impl Config for Data {
    type FormatType = JsonFormat;
    type FormatContext = ();

    fn config_path_and_filename(_home_dir: &std::path::Path) -> (Option<std::path::PathBuf>, &str) {
        (None, CONFIG_NAME)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Machine {
    pub name: String,
    pub mac: String,
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
