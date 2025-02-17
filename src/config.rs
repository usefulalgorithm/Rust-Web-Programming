use std::{env, fs};

use yaml_rust2::YamlLoader;

pub struct Config {
    pub db_url: String,
}

impl Config {
    pub fn new() -> Self {
        let args: Vec<String> = env::args().collect();
        let path = &args[args.len() - 1];
        let file = fs::read_to_string(path).unwrap();
        let db_url = YamlLoader::load_from_str(&file).unwrap()[0]["DB_URL"]
            .as_str()
            .unwrap()
            .to_string();
        Self { db_url }
    }
}
