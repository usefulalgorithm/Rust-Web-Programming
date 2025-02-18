use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Deserialize, Serialize, Debug)]
pub struct Counter {
    pub count: i32,
}

impl Counter {
    pub fn get_redis_url() -> String {
        Config::new().redis_url
    }
    pub fn save(self) -> Result<(), Box<dyn Error>> {
        let value = serde_json::to_vec(&self)?;
        let mut conn = redis::Client::open(Self::get_redis_url())?.get_connection()?;
        Ok(redis::cmd("SET")
            .arg("COUNTER")
            .arg(value)
            .exec(&mut conn)?)
    }
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let mut conn = redis::Client::open(Self::get_redis_url())?.get_connection()?;
        let data = redis::cmd("GET")
            .arg("COUNTER")
            .query::<Vec<u8>>(&mut conn)?;
        Ok(serde_json::from_slice::<Counter>(&data)?)
    }
}
