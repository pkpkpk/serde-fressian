

pub mod INST {

    use chrono::{ DateTime, Utc,};
    use chrono::offset::{TimeZone, Offset};
    use chrono::naive::{NaiveDateTime};

    use serde::de::{Deserializer, Deserialize};
    use serde::ser::{Serialize, Serializer, SerializeStruct};

    #[derive(Shrinkwrap)]
    pub struct INST (DateTime<Utc>);

    impl INST {
        // from mentat/edn
        pub fn from_millis(ms: i64) -> Self {
            INST(Utc.timestamp(ms / 1_000, ((ms % 1_000).abs() as u32) * 1_000))
        }
        pub fn to_millis(&self) -> i64 {
            let major: i64 = self.timestamp() * 1_000;
            let minor: i64 = self.timestamp_subsec_millis() as i64;
            major + minor
        }
        pub fn into_inner(self) -> DateTime<Utc> {
            self.0
        }
    }

    impl<'de> Deserialize<'de> for INST {
        fn deserialize<D>(deserializer: D) -> Result<INST, D::Error>
            where D: Deserializer<'de>,
        {
            let ms: i64 = i64::deserialize(deserializer)?;
            Ok(INST::from_millis(ms) )
        }
    }

    impl Serialize for INST {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_newtype_struct("INST", &self.to_millis())
        }
    }
}

pub mod UUID {

    use serde::de::{Deserializer, Deserialize, Error};
    use serde::ser::{Serialize, Serializer, SerializeStruct};
    use serde_bytes::ByteBuf;

    use uuid::Uuid;

    #[derive(Shrinkwrap)]
    pub struct UUID (Uuid);

    impl UUID {
        pub fn from_Uuid(u: Uuid) -> Self {
            UUID(u)
        }
        pub fn into_inner(self) -> Uuid {
            self.0
        }
    }

    impl<'de> Deserialize<'de> for UUID {
        fn deserialize<D>(deserializer: D) -> Result<UUID, D::Error>
            where D: Deserializer<'de>,
        {
            let bytes: ByteBuf = ByteBuf::deserialize(deserializer)?;

            match Uuid::from_bytes(bytes.as_ref()) {
                Ok(uuid) => Ok(UUID(uuid)),
                Err(e) => Err(Error::custom(e))
            }
        }
    }

    impl Serialize for UUID {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let bytes: &[u8] = self.as_bytes();
            let buf = ByteBuf::from(bytes);
            serializer.serialize_newtype_struct("UUID", &buf)
        }
    }
}

pub mod URI {
    use serde::de::{Deserializer, Deserialize, Error};
    use serde::ser::{Serialize, Serializer, SerializeStruct};
    use url::{Url};

    #[derive(Shrinkwrap)]
    pub struct URI (Url);

    impl URI {
        pub fn from_Url(u: Url) -> Self {
            URI(u)
        }
        pub fn into_inner(self) -> Url {
            self.0
        }
    }

    impl<'de> Deserialize<'de> for URI {
        fn deserialize<D>(deserializer: D) -> Result<URI, D::Error>
            where D: Deserializer<'de>,
        {
            let s: String = String::deserialize(deserializer)?;

            match Url::parse(s.as_ref()) {
                Ok(u) => Ok(URI(u)),
                Err(e) => Err(Error::custom(e))
            }
        }
    }

    impl Serialize for URI {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_newtype_struct("URI", self.as_str())
        }
    }
}

pub mod REGEX {
    /// might be able to get away with a remote attr here but for consistency
    /// just wrapping

    use serde::de::{Deserializer, Deserialize, Error};
    use serde::ser::{Serialize, Serializer, SerializeStruct};

    use regex::Regex;

    #[derive(Shrinkwrap)]
    pub struct REGEX (Regex);

    impl REGEX {
        pub fn from_Regex(re: Regex) -> Self {
            REGEX(re)
        }
        pub fn into_inner(self) -> Regex {
            self.0
        }
    }

    impl<'de> Deserialize<'de> for REGEX {
        fn deserialize<D>(deserializer: D) -> Result<REGEX, D::Error>
            where D: Deserializer<'de>,
        {
            let s: String = String::deserialize(deserializer)?;

            match Regex::new(s.as_ref()) {
                Ok(re) => Ok(REGEX::from_Regex(re)),
                Err(e) => Err(Error::custom(e))
            }
        }
    }

    impl Serialize for REGEX {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_newtype_struct("REGEX", self.0.as_str())
        }
    }
}