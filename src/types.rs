
pub mod inst {

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
    use serde::ser::{Serialize, Serializer};

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

#[cfg(not(use_regex_crate))]
pub mod uuid {
    use serde::de::{Deserializer, Deserialize};
    use serde::ser::{Serialize, Serializer};
    use serde_bytes::ByteBuf;

    #[derive(Shrinkwrap, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Debug)]
    pub struct UUID (ByteBuf);

    impl UUID {
        pub fn into_inner(self) -> ByteBuf {
            self.0
        }

        #[inline]
        pub fn from_bytes(bytes: &[u8]) -> Result<Self, ()> {
            let buf = ByteBuf::from(bytes);
            Ok(UUID(buf))
        }
    }

    impl<'de> Deserialize<'de> for UUID {
        fn deserialize<D>(deserializer: D) -> Result<UUID, D::Error>
            where D: Deserializer<'de>,
        {
            let buf: ByteBuf = ByteBuf::deserialize(deserializer)?;

            Ok(UUID(buf))
        }
    }

    impl Serialize for UUID {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let bytes: &[u8] = self.0.as_ref();
            let buf = ByteBuf::from(bytes);
            serializer.serialize_newtype_struct("UUID", &buf)
        }
    }
}

#[cfg(use_regex_crate)]
pub mod uuid {

    use serde::de::{Deserializer, Deserialize};
    use serde::ser::{Serialize, Serializer};
    use serde_bytes::ByteBuf;

    use _uuid::Uuid;

    #[derive(Shrinkwrap, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Debug)]
    pub struct UUID (Uuid);

    impl UUID {
        pub fn from_uuid(u: Uuid) -> Self {
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

#[cfg(not(use_url_crate))]
pub mod uri {
    use serde::de::{Deserializer, Deserialize};
    use serde::ser::{Serialize, Serializer};

    #[derive(Shrinkwrap, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Debug)]
    pub struct URI (String);

    impl URI {
        pub fn new(s: String) -> Self {
            URI(s)
        }

        pub fn into_inner(self) -> String {
            self.0
        }

        #[inline]
        pub fn from_str(s: &str) -> Result<Self, ()> {
            Ok(URI(s.to_string()))
        }
    }
    impl<'de> Deserialize<'de> for URI {
        fn deserialize<D>(deserializer: D) -> Result<URI, D::Error>
            where D: Deserializer<'de>,
        {
            let s: String = String::deserialize(deserializer)?;

            Ok(URI(s))
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

#[cfg(use_url_crate)]
pub mod uri {
    use serde::de::{Deserializer, Deserialize, Error};
    use serde::ser::{Serialize, Serializer};
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

#[cfg(not(use_regex_crate))]
pub mod regex {

    #[derive(Shrinkwrap, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct REGEX (pub String);

    impl REGEX {
        pub fn into_inner(self) -> String {
            self.0
        }
        pub fn from_str(s: &str) -> Result<Self, ()> {
            Ok(REGEX(s.to_string()))
        }
    }

    use serde::de::{Deserializer, Deserialize};
    use serde::ser::{Serialize, Serializer};

    impl<'de> Deserialize<'de> for REGEX {
        fn deserialize<D>(deserializer: D) -> Result<REGEX, D::Error>
            where D: Deserializer<'de>,
        {
            let s: String = String::deserialize(deserializer)?;

            Ok(REGEX(s))
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

#[cfg(use_regex_crate)]
pub mod regex {
    use _regex::Regex;
    use std::cmp::{Eq,PartialOrd, Ordering};

    #[derive(Shrinkwrap, Clone, Debug)]
    pub struct REGEX (Regex);

    impl REGEX {
        pub fn from_regex(re: Regex) -> Self {
            REGEX(re)
        }

        pub fn into_inner(self) -> Regex {
            self.0
        }

        #[inline]
        pub fn from_str(s: &str) -> Result<Self, regex::Error> {
            Regex::new(s).map(REGEX::from_regex)
        }
    }

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

    use serde::de::{Deserializer, Deserialize, Error};
    use serde::ser::{Serialize, Serializer};

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



pub mod sym {

    use serde::ser::{Serialize, Serializer};

    #[derive(Clone,Debug,Eq,Hash,Ord,PartialOrd,PartialEq,Deserialize)]
    pub struct SYM(Option<String>,String);

    impl SYM {
        pub fn new(namespace: Option<String>, name: String) -> Self {
            SYM(namespace, name)
        }
        pub fn simple(name: String) -> Self {
            SYM(None, name)
        }
        pub fn namespaced(namespace: String, name: String) -> Self {
            SYM(Some(namespace), name)
        }
    }

    use serde::ser::SerializeTupleStruct;

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

pub mod key {

    use serde::ser::{Serialize, Serializer};

    //same as SYM above
    #[derive(Clone,Debug,Eq,Hash,Ord,PartialOrd,PartialEq, Deserialize)]
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

    use serde::ser::SerializeTupleStruct;

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

    use serde::de::{Deserializer, Deserialize};
    use serde::ser::{Serialize, Serializer};

    #[derive(Shrinkwrap,Clone,Debug,Eq,Hash,Ord,PartialOrd,PartialEq)]
    pub struct IntArray (Vec<i32>);

    impl IntArray {
        pub fn from_vec(v: Vec<i32>) -> Self {
            IntArray(v)
        }
    }

    impl From<Vec<i32>> for IntArray {
        #[inline]
        fn from(val: Vec<i32>) -> IntArray {
            IntArray::from_vec(val)
        }
    }

    impl<'de> Deserialize<'de> for IntArray {
        fn deserialize<D>(deserializer: D) -> Result<IntArray, D::Error>
            where D: Deserializer<'de>,
        {
            let v: Vec<i32> = Vec::deserialize(deserializer)?;

            Ok(IntArray(v))
        }
    }

    impl Serialize for IntArray {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_newtype_struct("INT_ARRAY", &self.0)
        }
    }

    pub mod int_array {
        use serde::ser::{Serializer};

        pub fn serialize<S>(vec: &Vec<i32>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_newtype_struct("INT_ARRAY", vec)
        }
    }

    #[derive(Shrinkwrap,Clone,Debug,Eq,Hash,Ord,PartialOrd,PartialEq)]
    pub struct LongArray (Vec<i64>);

    impl LongArray {
        pub fn from_vec(v: Vec<i64>) -> Self {
            LongArray(v)
        }
    }

    impl From<Vec<i64>> for LongArray {
        #[inline]
        fn from(val: Vec<i64>) -> LongArray {
            LongArray::from_vec(val)
        }
    }

    impl<'de> Deserialize<'de> for LongArray {
        fn deserialize<D>(deserializer: D) -> Result<LongArray, D::Error>
            where D: Deserializer<'de>,
        {
            let v: Vec<i64> = Vec::deserialize(deserializer)?;

            Ok(LongArray(v))
        }
    }

    impl Serialize for LongArray {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_newtype_struct("LONG_ARRAY", &self.0)
        }
    }

    pub mod long_array {
        use serde::ser::{Serializer};

        pub fn serialize<S>(vec: &Vec<i64>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_newtype_struct("LONG_ARRAY", vec)
        }
    }

    use ordered_float::OrderedFloat;

    #[derive(Shrinkwrap, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Debug)]
    pub struct FloatArray (Vec<OrderedFloat<f32>>);

    impl FloatArray {
        pub fn from_vec(v: Vec<f32>) -> Self {
            let ret: Vec<OrderedFloat<f32>> = v.into_iter()
                                               .map(|f: f32| OrderedFloat::from(f))
                                               .collect();
            FloatArray(ret)
        }
    }

    impl From<Vec<f32>> for FloatArray {
        #[inline]
        fn from(val: Vec<f32>) -> FloatArray {
            FloatArray::from_vec(val)
        }
    }

    impl<'de> Deserialize<'de> for FloatArray {
        fn deserialize<D>(deserializer: D) -> Result<FloatArray, D::Error>
            where D: Deserializer<'de>,
        {
            let v: Vec<OrderedFloat<f32>> = Vec::deserialize(deserializer)?;

            Ok(FloatArray(v))
        }
    }

    impl Serialize for FloatArray {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_newtype_struct("FLOAT_ARRAY", &self.0)
        }
    }

    pub mod float_array {
        use serde::ser::{Serializer};

        pub fn serialize<S>(vec: &Vec<f32>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_newtype_struct("FLOAT_ARRAY", vec)
        }
    }

    #[derive(Shrinkwrap, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Debug)]
    pub struct DoubleArray (Vec<OrderedFloat<f64>>);

    impl DoubleArray {
        pub fn from_vec(v: Vec<f64>) -> Self {
            let ret: Vec<OrderedFloat<f64>> = v.into_iter()
                                               .map(|f: f64| OrderedFloat::from(f))
                                               .collect();
            DoubleArray(ret)
        }
    }

    impl From<Vec<f64>> for DoubleArray {
        #[inline]
        fn from(val: Vec<f64>) -> DoubleArray {
            DoubleArray::from_vec(val)
        }
    }

    impl<'de> Deserialize<'de> for DoubleArray {
        fn deserialize<D>(deserializer: D) -> Result<DoubleArray, D::Error>
            where D: Deserializer<'de>,
        {
            let v: Vec<OrderedFloat<f64>> = Vec::deserialize(deserializer)?;

            Ok(DoubleArray(v))
        }
    }

    impl Serialize for DoubleArray {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_newtype_struct("DOUBLE_ARRAY", &self.0)
        }
    }

    pub mod double_array {
        use serde::ser::{Serializer};

        pub fn serialize<S>(vec: &Vec<f64>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_newtype_struct("DOUBLE_ARRAY", vec)
        }
    }


    #[derive(Shrinkwrap,Clone,Debug,Eq,Hash,Ord,PartialOrd,PartialEq)]
    pub struct BooleanArray (Vec<bool>);

    impl BooleanArray {
        pub fn from_vec(v: Vec<bool>) -> Self {
            BooleanArray(v)
        }
    }

    impl From<Vec<bool>> for BooleanArray {
        #[inline]
        fn from(val: Vec<bool>) -> BooleanArray {
            BooleanArray::from_vec(val)
        }
    }

    impl<'de> Deserialize<'de> for BooleanArray {
        fn deserialize<D>(deserializer: D) -> Result<BooleanArray, D::Error>
            where D: Deserializer<'de>,
        {
            let v: Vec<bool> = Vec::deserialize(deserializer)?;

            Ok(BooleanArray(v))
        }
    }

    impl Serialize for BooleanArray {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_newtype_struct("BOOLEAN_ARRAY", &self.0)
        }
    }

    pub mod boolean_array {
        use serde::ser::{Serializer};

        pub fn serialize<S>(vec: &Vec<bool>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_newtype_struct("BOOLEAN_ARRAY", vec)
        }
    }
}

pub mod set {

    use serde::ser::{Serialize, Serializer};
    use std::collections::{BTreeSet, HashSet};
    use std::hash::Hash;
    use std::cmp::{Ord};

    pub fn serialize<I, V, S>(set: &I, serializer: S) -> Result<S::Ok, S::Error>
    where
        V: Serialize + Eq + Hash,
        I: IntoIterator<Item = V> + Serialize,
        S: Serializer,
    {
        serializer.serialize_newtype_struct("SET", set)
    }

    #[derive(Shrinkwrap,Clone,Debug,Eq,Hash,Ord,PartialOrd,PartialEq, Deserialize)]
    pub struct SET<T: Ord>(BTreeSet<T>);

    impl<T: Ord> SET<T> {
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

    #[derive(Shrinkwrap,Clone,Deserialize)]
    pub struct HASHSET<T: Ord + Hash>(HashSet<T>);

    impl<T: Ord + Hash> HASHSET<T> {
        pub fn into_inner(self) -> HashSet<T> {
            self.0
        }
    }

    impl<T: Ord + Hash> From<HashSet<T>> for HASHSET<T>
    {
        #[inline]
        fn from(val: HashSet<T>) -> HASHSET<T> {
            HASHSET(val)
        }
    }

    impl<T> Serialize for HASHSET<T>
        where T: Serialize + Ord + std::hash::Hash,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,

        {
            serializer.serialize_newtype_struct("SET", &self.0)
        }
    }
}

