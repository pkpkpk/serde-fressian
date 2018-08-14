

pub mod inst {
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
}

// ($f, "#uuid \"{}\"", u.hyphenated().to_string()
// :Uuid::parse_str("4cb3f828-752d-497a-90c9-b1fd516d5644").expect("valid uuid");

    // def_into!(into_uuid, $t::Uuid, Uuid,);
    // def_as_ref!(as_uuid, $t::Uuid, Uuid);


pub mod UUID {
    use serde;
    use serde::de::{Deserializer, Deserialize, Error};

    use serde::ser::{Serialize, Serializer, SerializeStruct};

    use serde_bytes::ByteBuf;

    // use imp::error::{Error};
    use imp::codes;
    use uuid::Uuid;

    #[derive(Shrinkwrap)]
    pub struct UUID (Uuid);

    impl<'de> Deserialize<'de> for UUID {
        fn deserialize<D>(deserializer: D) -> Result<UUID, D::Error>
            where D: Deserializer<'de>,
        {
            // let bytes: &[u8] = &[u8]::deserialize(deserializer)?;
            // let buf =  ByteBuf.deserialize(deserializer)?;

            let bytes: ByteBuf = ByteBuf::deserialize(deserializer)?;

            // let bytes = buf.as_slice();

            let mut offset = 0;
            let mut acc: Vec<String> = vec![];

            for n in vec![4, 2, 2, 2, 6].into_iter(){
                let token: String = bytes.iter()
                                    .skip(offset)
                                    .take(n)
                                    .map(|i: &u8| format!("{:02X}", *i + 0x100))
                                    .map(|s: String| s.chars().skip(1).collect::<String>())
                                    .collect();
                offset += n;
                acc.push(token);
            }

            match Uuid::parse_str(acc.join("-").as_ref()) {
                Ok(uuid) => Ok(UUID(uuid)),
                // Err(_) => Err(Error::Message("bad uuid".to_string()))
                Err(e) => Err(Error::custom(e))
            }
        }
    }



    // let bytes: &[u8] = self.rawIn.read_bytes()?;
    //
    // let mut offset = 0;
    // let mut acc: Vec<String> = vec![];
    // for n in vec![4, 2, 2, 2, 6].into_iter(){
    //     let token: String = bytes.iter()
    //                         .skip(offset)
    //                         .take(n)
    //                         .map(|i: &u8| format!("{:02X}", *i + 0x100))
    //                         .map(|s: String| s.chars().skip(1).collect())
    //                         .collect();
    //     offset += n;
    //     acc.push(token);
    // }
    //
    // match Uuid::parse_str(acc.join("-").as_ref()) {
    //     Ok(uuid) => Ok(uuid),
    //     Err(_) => Err(Error::Message("bad uuid".to_string()))
    // }
}