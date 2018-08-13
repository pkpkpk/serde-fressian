use chrono::{ DateTime, Utc,};
use chrono::offset::{TimeZone, Offset};
use chrono::naive::{NaiveDateTime};

// use serde_derive::Deserialize;
use serde;
use serde::de::{Deserializer, Deserialize};

use imp::error::{Error};


// from mentat/edn
pub trait FromMillis {
    fn from_millis(ts: i64) -> Self;
}

impl FromMillis for DateTime<Utc> {
    fn from_millis(ts: i64) -> Self {
        Utc.timestamp(ts / 1_000, ((ts % 1_000).abs() as u32) * 1_000)
    }
}


// #[serde(remote = "DateTime")] // chrono::DateTime
// pub struct DateTimeAlias<Tz: TimeZone> {
//     datetime: NaiveDateTime,
//     offset: Tz::Offset,
// }

#[derive(Shrinkwrap)]
pub struct Inst (DateTime<Utc>);


impl<'de> Deserialize<'de> for Inst {
    fn deserialize<D>(deserializer: D) -> Result<Inst, D::Error>
        where D: Deserializer<'de>,
    {
        // let ms: i64 = deserializer.deserialize_any()?;
        let ms: i64 = i64::deserialize(deserializer)?;
        let t = DateTime::from_millis(ms);
        Ok(Inst(t))
    }
}