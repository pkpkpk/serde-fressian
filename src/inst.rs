use chrono::{ DateTime, Utc,};
use chrono::offset::{TimeZone, Offset};
use chrono::naive::{NaiveDateTime};

// use serde_derive::Deserialize;
use serde;
use serde::de::{Deserializer, Deserialize};
use serde::ser::{Serialize, Serializer, SerializeStruct};

use imp::error::{Error};
use imp::codes;

#[derive(Shrinkwrap)]
pub struct Inst (DateTime<Utc>);

impl Inst {
    // from mentat/edn
    pub fn from_millis(ms: i64) -> Self {
        Inst(Utc.timestamp(ms / 1_000, ((ms % 1_000).abs() as u32) * 1_000))
    }
    pub fn to_millis(&self) -> i64 {
        let major: i64 = self.timestamp() * 1_000;
        let minor: i64 = self.timestamp_subsec_millis() as i64;
        major + minor
    }
}

impl<'de> Deserialize<'de> for Inst {
    fn deserialize<D>(deserializer: D) -> Result<Inst, D::Error>
        where D: Deserializer<'de>,
    {
        let ms: i64 = i64::deserialize(deserializer)?;
        // let t = DateTime::from_millis(ms);
        Ok(Inst::from_millis(ms) )
    }
}

impl Serialize for Inst {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // let ms =  self.timestamp_millis();
        let ms = self.to_millis();
        serializer.serialize_newtype_struct("INST", &ms)
    }
}