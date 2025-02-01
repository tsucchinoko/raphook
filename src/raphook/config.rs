use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(flatten)]
    pub hooks: HashMap<String, Hook>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hook {
    pub parallel: Option<bool>,
    pub commands: HashMap<String, Command>,
    pub scripts: Option<HashMap<String, Script>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Command {
    pub tags: Option<String>,
    pub run: String,
    pub glob: Option<String>,
    pub files: Option<String>,
    pub exclude: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Script {
    pub runner: String,
}

impl Config {
    pub fn load(path: &str) -> io::Result<Self> {
        let config_file_path = Path::new(path).join("raphook.yml");
        let config_file = File::open(config_file_path).unwrap();
        let reader = BufReader::new(config_file);

        let config: Config = serde_yaml::from_reader(reader)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        Ok(config)
    }

    pub fn hook_names(&self) -> Vec<&str> {
        self.hooks.keys().map(|s| s.as_str()).collect()
    }
}
