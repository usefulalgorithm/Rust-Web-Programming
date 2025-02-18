use std::{env, fs};

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Config {
    pub db_url: String,
    pub secret_key: String,
    pub expire_minutes: i64,
    pub redis_url: String,
}

impl Config {
    pub fn new() -> Self {
        if cfg!(test) {
            Self {
                db_url: "postgres://username:password@localhost:5433/to_do".to_string(),
                secret_key: "secret".to_string(),
                expire_minutes: 120,
                redis_url: "redis://127.0.0.1:6379".to_string(),
            }
        } else {
            let args: Vec<String> = env::args().collect();
            let path = &args[args.len() - 1];
            let file = fs::read_to_string(path).unwrap();
            serde_yaml::from_str::<Config>(&file).unwrap()
        }
    }
}
