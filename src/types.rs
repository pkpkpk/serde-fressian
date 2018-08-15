

pub mod inst {

    use chrono::{ DateTime, Utc,};
    use chrono::offset::{TimeZone, Offset};
    use chrono::naive::{NaiveDateTime};

    use serde::de::{Deserializer, Deserialize};
    use serde::ser::{Serialize, Serializer, SerializeStruct};

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
    }

    impl<'de> Deserialize<'de> for UUID {
        fn deserialize<D>(deserializer: D) -> Result<UUID, D::Error>
            where D: Deserializer<'de>,
        {
            let bytes: ByteBuf = ByteBuf::deserialize(deserializer)?;

            let mut offset = 0;
            let mut acc: Vec<String> = vec![];

            for n in vec![4, 2, 2, 2, 6].into_iter(){
                let token: String = bytes.iter()
                                    .skip(offset)
                                    .take(n)
                                    .map(|i: &u8| format!("{:02X}", *i as u32 + 0x100))
                                    .map(|s: String| s.chars().skip(1).collect::<String>())
                                    .collect();
                offset += n;
                acc.push(token);
            }

            match Uuid::parse_str(acc.join("-").as_ref()) {
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
