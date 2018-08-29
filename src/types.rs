

pub mod INST {

    // was using chrono Datetime<Utc> because thats what mentat used
    // but it appears that Utc is a lossy representation.
    // clojure insts are ~rfc3339; see https://clojure.github.io/clojure/clojure.instant-api.html
    // Chrono has from from_rfc string methods but need to write
    // roundtrip lossless Datetime<Fixed> <--> millisec

    // probably should be hid under a feature flag any way.
    //  - most users will not use
    //  - need bindings for richer functionality

    // use chrono::{ DateTime, Utc,};
    // use chrono::offset::{TimeZone, Offset};
    // use chrono::naive::{NaiveDateTime};

    // #[derive(Shrinkwrap, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Debug)]
    // pub struct INST (DateTime<Utc>);
    //
    // impl INST {
    //     // from mentat/edn
    //     pub fn from_millis(ms: i64) -> Self {
    //         INST(Utc.timestamp(ms / 1_000, ((ms % 1_000).abs() as u32) * 1_000))
    //     }
    //     pub fn to_millis(&self) -> i64 {
    //         let major: i64 = self.timestamp() * 1_000;
    //         let minor: i64 = self.timestamp_subsec_millis() as i64;
    //         major + minor
    //     }
    //     pub fn into_inner(self) -> DateTime<Utc> {
    //         self.0
    //     }
    // }

    use serde::de::{Deserializer, Deserialize};
    use serde::ser::{Serialize, Serializer, SerializeStruct};

    #[derive(Shrinkwrap, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Debug)]
    pub struct INST (i64);

    impl INST {
        pub fn from_millis(ms: i64) -> Self {
            INST(ms)
        }
        pub fn to_millis(&self) -> i64 {
            self.0
        }
        pub fn into_inner(self) -> i64 {
            self.0
        }
    }

    impl<'de> Deserialize<'de> for INST {
        fn deserialize<D>(deserializer: D) -> Result<INST, D::Error>
            where D: Deserializer<'de>,
        {
            let ms: i64 = i64::deserialize(deserializer)?;
            Ok(INST::from_millis(ms))
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

    #[derive(Shrinkwrap, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Debug)]
    pub struct UUID (Uuid);

    impl UUID {
        pub fn from_Uuid(u: Uuid) -> Self {
            UUID(u)
        }
        pub fn into_inner(self) -> Uuid {
            self.0
        }

        #[inline]
        pub fn from_bytes(bytes: &[u8]) -> Result<Self, uuid::ParseError> {
            Uuid::from_bytes(bytes).map(UUID::from_Uuid)
        }
    }

    impl<'de> Deserialize<'de> for UUID {
        fn deserialize<D>(deserializer: D) -> Result<UUID, D::Error>
            where D: Deserializer<'de>,
        {
            let bb: ByteBuf = ByteBuf::deserialize(deserializer)?;

            UUID::from_bytes(bb.as_ref()).map_err(Error::custom)
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

    #[derive(Shrinkwrap, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Debug)]
    pub struct URI (Url);

    impl URI {
        pub fn from_Url(u: Url) -> Self {
            URI(u)
        }
        pub fn into_inner(self) -> Url {
            self.0
        }

        #[inline]
        pub fn from_str(s: &str) -> Result<Self, url::ParseError> {
            Url::parse(s).map(URI::from_Url)
        }
    }

    impl<'de> Deserialize<'de> for URI {
        fn deserialize<D>(deserializer: D) -> Result<URI, D::Error>
            where D: Deserializer<'de>,
        {
            let s: String = String::deserialize(deserializer)?;

            URI::from_str(s.as_ref()).map_err(Error::custom)
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

    #[derive(Shrinkwrap, Clone, Debug)] //PartialOrd, Eq
    pub struct REGEX (Regex);

    impl REGEX {
        pub fn from_Regex(re: Regex) -> Self {
            REGEX(re)
        }
        pub fn into_inner(self) -> Regex {
            self.0
        }

        #[inline]
        pub fn from_str(s: &str) -> Result<Self, regex::Error> {
            Regex::new(s).map(REGEX::from_Regex)
        }
    }

    use std::cmp::{Eq,PartialOrd, Ordering};

    impl Ord for REGEX {
        fn cmp(&self, other: &REGEX) -> Ordering {
            self.as_str().cmp(other.as_str())
        }
    }

    impl PartialOrd for REGEX {
        fn partial_cmp(&self, other: &REGEX) -> Option<Ordering> {
             Some(self.as_str().cmp(other.as_str()))
        }
    }

    impl PartialEq for REGEX {
        fn eq(&self, other: &REGEX) -> bool {
            self.as_str() == other.as_str()
        }
    }

    impl Eq for REGEX {}

    use std::hash::{Hash, Hasher};

    impl Hash for REGEX {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.as_str().hash(state)
        }
    }

    impl<'de> Deserialize<'de> for REGEX {
        fn deserialize<D>(deserializer: D) -> Result<REGEX, D::Error>
            where D: Deserializer<'de>,
        {
            let s: String = String::deserialize(deserializer)?;

            REGEX::from_str(s.as_ref()).map_err(Error::custom)
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


pub mod SYM {

    use serde::de::{Deserializer, Deserialize, Error};
    use serde::ser::{Serialize, Serializer};

    // TODO investigate https://github.com/mozilla/mentat/blob/master/edn/src/symbols.rs
    // going simple for now
    // not clear if any advantages to full struct
    #[derive(Clone,Debug,Eq,Hash,Ord,PartialOrd,PartialEq)]
    pub struct SYM(Option<String>,String);
    // pub struct SYM {
    //     namespace: String,
    //     name: String
    // }

    impl SYM {
        pub fn new(namespace: Option<String>, name: String) -> Self {
            SYM(namespace, name)
            // SYM {
            //     namespace: namespace,
            //     name: name
            // }
        }
        pub fn simple(name: String) -> Self {
            SYM(None, name)
        }
        pub fn namespaced(namespace: String, name: String) -> Self {
            SYM(Some(namespace), name)
        }
    }

    impl<'de> Deserialize<'de> for SYM {
        fn deserialize<D>(deserializer: D) -> Result<SYM, D::Error>
            where D: Deserializer<'de>,
        {
            // is vector right way to do this?
            // [namespace, name]
            let mut v: Vec< Option<String> > = Vec::deserialize(deserializer)?;

            let name: Option<Option<String>> = v.pop();
            let namespace: Option<Option<String>> = v.pop();

            match (namespace,name) {
                (Some(namespace_opt),Some(Some(name_string))) => {
                    Ok(SYM::new(namespace_opt, name_string))
                }
                _ => Err(Error::custom("bad symbol"))
            }
        }
    }

    use serde::ser::SerializeTupleStruct;
    use imp::codes;

    impl Serialize for SYM {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut state = serializer.serialize_tuple_struct("SYM", 2)?;
            let namespace: &Option<String> = &self.0;
            let name: &String = &self.1;

            state.serialize_field(namespace)?;
            state.serialize_field(name)?;
            state.end()
        }
    }
}

pub mod KEY {

    use serde::de::{Deserializer, Deserialize, Error};
    use serde::ser::{Serialize, Serializer};

    //same as SYM above
    #[derive(Clone,Debug,Eq,Hash,Ord,PartialOrd,PartialEq)]
    pub struct KEY(Option<String>,String);

    impl KEY {
        pub fn new(namespace: Option<String>, name: String) -> Self {
            KEY(namespace, name)
        }
        pub fn simple(name: String) -> Self {
            KEY(None, name)
        }
        pub fn namespaced(namespace: String, name: String) -> Self {
            KEY(Some(namespace), name)
        }
    }

    impl<'de> Deserialize<'de> for KEY {
        fn deserialize<D>(deserializer: D) -> Result<KEY, D::Error>
            where D: Deserializer<'de>,
        {
            // [namespace, name]
            let mut v: Vec< Option<String>> = Vec::deserialize(deserializer)?;

            let name: Option<Option<String>> = v.pop();
            let namespace: Option<Option<String>> = v.pop();

            match (namespace,name) {
                (Some(namespace_opt),Some(Some(name_string))) => {
                    Ok(KEY::new(namespace_opt, name_string))
                }
                _ => Err(Error::custom("bad keyword"))
            }
        }
    }

    use serde::ser::SerializeTupleStruct;
    use imp::codes;

    impl Serialize for KEY {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut state = serializer.serialize_tuple_struct("KEY", 2)?;
            state.serialize_field(&self.0)?;
            state.serialize_field(&self.1)?;
            state.end()
        }
    }
}

pub mod typed_arrays {

    use serde::de::{Deserializer, Deserialize, Error};
    use serde::ser::{Serialize, Serializer};

    #[derive(Shrinkwrap,Clone,Debug,Eq,Hash,Ord,PartialOrd,PartialEq)]
    pub struct Int_Array (Vec<i32>);

    impl Int_Array {
        pub fn from_vec(v: Vec<i32>) -> Self {
            Int_Array(v)
        }
    }

    impl From<Vec<i32>> for Int_Array {
        #[inline]
        fn from(val: Vec<i32>) -> Int_Array {
            Int_Array::from_vec(val)
        }
    }

    impl<'de> Deserialize<'de> for Int_Array {
        fn deserialize<D>(deserializer: D) -> Result<Int_Array, D::Error>
            where D: Deserializer<'de>,
        {
            let v: Vec<i32> = Vec::deserialize(deserializer)?;

            Ok(Int_Array(v))
        }
    }

    impl Serialize for Int_Array {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_newtype_struct("INT_ARRAY", &self.0)
        }
    }


    #[derive(Shrinkwrap,Clone,Debug,Eq,Hash,Ord,PartialOrd,PartialEq)]
    pub struct Long_Array (Vec<i64>);

    impl Long_Array {
        pub fn from_vec(v: Vec<i64>) -> Self {
            Long_Array(v)
        }
    }

    impl From<Vec<i64>> for Long_Array {
        #[inline]
        fn from(val: Vec<i64>) -> Long_Array {
            Long_Array::from_vec(val)
        }
    }

    impl<'de> Deserialize<'de> for Long_Array {
        fn deserialize<D>(deserializer: D) -> Result<Long_Array, D::Error>
            where D: Deserializer<'de>,
        {
            let v: Vec<i64> = Vec::deserialize(deserializer)?;

            Ok(Long_Array(v))
        }
    }

    impl Serialize for Long_Array {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_newtype_struct("LONG_ARRAY", &self.0)
        }
    }

    use ordered_float::OrderedFloat;

    #[derive(Shrinkwrap, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Debug)]
    pub struct Float_Array (Vec<OrderedFloat<f32>>);

    impl Float_Array {
        pub fn from_vec(v: Vec<f32>) -> Self {
            let ret: Vec<OrderedFloat<f32>> = v.into_iter()
                                               .map(|f: f32| OrderedFloat::from(f))
                                               .collect();
            Float_Array(ret)
        }
    }

    impl From<Vec<f32>> for Float_Array {
        #[inline]
        fn from(val: Vec<f32>) -> Float_Array {
            Float_Array::from_vec(val)
        }
    }

    impl<'de> Deserialize<'de> for Float_Array {
        fn deserialize<D>(deserializer: D) -> Result<Float_Array, D::Error>
            where D: Deserializer<'de>,
        {
            let v: Vec<OrderedFloat<f32>> = Vec::deserialize(deserializer)?;

            Ok(Float_Array(v))
        }
    }

    impl Serialize for Float_Array {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_newtype_struct("FLOAT_ARRAY", &self.0)
        }
    }

    #[derive(Shrinkwrap, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Debug)]
    pub struct Double_Array (Vec<OrderedFloat<f64>>);

    impl Double_Array {
        pub fn from_vec(v: Vec<f64>) -> Self {
            let ret: Vec<OrderedFloat<f64>> = v.into_iter()
                                               .map(|f: f64| OrderedFloat::from(f))
                                               .collect();
            Double_Array(ret)
        }
    }

    impl From<Vec<f64>> for Double_Array {
        #[inline]
        fn from(val: Vec<f64>) -> Double_Array {
            Double_Array::from_vec(val)
        }
    }

    impl<'de> Deserialize<'de> for Double_Array {
        fn deserialize<D>(deserializer: D) -> Result<Double_Array, D::Error>
            where D: Deserializer<'de>,
        {
            let v: Vec<OrderedFloat<f64>> = Vec::deserialize(deserializer)?;

            Ok(Double_Array(v))
        }
    }

    impl Serialize for Double_Array {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_newtype_struct("DOUBLE_ARRAY", &self.0)
        }
    }


    #[derive(Shrinkwrap,Clone,Debug,Eq,Hash,Ord,PartialOrd,PartialEq)]
    pub struct Boolean_Array (Vec<bool>);

    impl Boolean_Array {
        pub fn from_vec(v: Vec<bool>) -> Self {
            Boolean_Array(v)
        }
    }

    impl From<Vec<bool>> for Boolean_Array {
        #[inline]
        fn from(val: Vec<bool>) -> Boolean_Array {
            Boolean_Array::from_vec(val)
        }
    }

    impl<'de> Deserialize<'de> for Boolean_Array {
        fn deserialize<D>(deserializer: D) -> Result<Boolean_Array, D::Error>
            where D: Deserializer<'de>,
        {
            let v: Vec<bool> = Vec::deserialize(deserializer)?;

            Ok(Boolean_Array(v))
        }
    }

    impl Serialize for Boolean_Array {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_newtype_struct("BOOLEAN_ARRAY", &self.0)
        }
    }
}

pub mod SET {

    use serde::de::{Deserializer, Deserialize, Error};
    use serde::ser::{Serialize, Serializer};
    use std::collections::{BTreeSet};
    use std::cmp::{Ord};

    #[derive(Shrinkwrap,Clone,Debug,Eq,Hash,Ord,PartialOrd,PartialEq)]
    pub struct SET<T>(BTreeSet<T>);

    impl<T> SET<T> {
        pub fn into_inner(self) -> BTreeSet<T> {
            self.0
        }
    }

    impl<T> From<BTreeSet<T>> for SET<T>
        where T: Serialize + Ord,
    {
        #[inline]
        fn from(val: BTreeSet<T>) -> SET<T> {
            SET(val)
        }
    }

    impl<T> Serialize for SET<T>
        where T: Serialize + Ord,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,

        {
            serializer.serialize_newtype_struct("SET", &self.0)
        }
    }

    impl<'de,T> Deserialize<'de> for SET<T>
        where T: Deserialize<'de> + Ord + Serialize,
    {
        fn deserialize<D>(deserializer: D) -> Result<SET<T>, D::Error>
            where D: Deserializer<'de>,
        {
            let v: BTreeSet<T> = BTreeSet::deserialize(deserializer)?;

            Ok(SET::from(v))
        }
    }
}

// into_inner methods