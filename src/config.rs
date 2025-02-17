use std::{env, fs};

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Config {
    pub db_url: String,
    pub secret_key: String,
    pub expire_minutes: i64,
}

impl Config {
    pub fn new() -> Self {
        let args: Vec<String> = env::args().collect();
        let path = &args[args.len() - 1];
        let file = fs::read_to_string(path).unwrap();
        serde_yaml::from_str::<Config>(&file).unwrap()
    }
}
