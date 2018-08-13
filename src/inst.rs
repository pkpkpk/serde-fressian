use chrono::{ DateTime, Utc,};
use chrono::offset::{TimeZone, Offset};
use chrono::naive::{NaiveDateTime};

// use serde_derive::Deserialize;
use serde;
use serde::de::{Deserializer, Deserialize};

use imp::error::{Error};


// from mentat/edn
pub trait FromMillis {
    fn from_millis(ms: i64) -> Self;
}

impl FromMillis for DateTime<Utc> {
    fn from_millis(ms: i64) -> Self {
        Utc.timestamp(ms / 1_000, ((ms % 1_000).abs() as u32) * 1_000)
    }
}

impl FromMillis for Inst {
    fn from_millis(ms: i64) -> Self {
        Inst(DateTime::from_millis(ms))
    }
}

#[derive(Shrinkwrap)]
pub struct Inst (DateTime<Utc>);

impl<'de> Deserialize<'de> for Inst {
    fn deserialize<D>(deserializer: D) -> Result<Inst, D::Error>
        where D: Deserializer<'de>,
    {
        let ms: i64 = i64::deserialize(deserializer)?;
        // let t = DateTime::from_millis(ms);
        Ok(Inst::from_millis(ms) )
    }
}

