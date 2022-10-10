use std::str::FromStr;

use serde::{de::Error, Deserialize, Deserializer};
use toml::value::Datetime;
use url::Url;

pub fn read_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    match u64::from_str(s) {
        Ok(o) => Ok(o),
        Err(e) => Err(Error::custom(format!("{}", e))),
    }
}

pub fn read_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    match u32::from_str(s) {
        Ok(o) => Ok(o),
        Err(e) => Err(Error::custom(format!("{}", e))),
    }
}

pub fn read_url<'de, D>(deserializer: D) -> Result<Url, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    match Url::from_str(s) {
        Ok(o) => Ok(o),
        Err(e) => Err(Error::custom(format!("{}", e))),
    }
}

pub fn read_date<'de, D>(deserializer: D) -> Result<Datetime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    match Datetime::from_str(s) {
        Ok(o) => Ok(o),
        Err(e) => Err(Error::custom(format!("{}", e))),
    }
}
