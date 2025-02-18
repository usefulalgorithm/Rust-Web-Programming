use std::io::Write;
use std::{fs::File, io::Read};

use anyhow::Ok;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct BuildConfig {
    allowed_version: String,
}

fn main() -> anyhow::Result<()> {
    let mut buf: String = String::new();
    let _ = File::open("./build_config.yml")?.read_to_string(&mut buf)?;
    let build_config = serde_yaml::from_str::<BuildConfig>(&buf)?;
    let mut output = File::create("./src/output_data.txt")?;
    write!(output, "{}", build_config.allowed_version)?;
    Ok(())
}
