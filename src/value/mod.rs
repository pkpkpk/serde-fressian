// /// Represents a JSON value
#[derive(Clone, PartialEq, PartialOrd)]
pub enum FressianValue {
    Null,
    Bool(bool),
    Int(i64),
    Float(f32),
    Double(f32),
    String(String),
    // Str(&str),
    // Array(Vec<Value>),
    // Object(BTreeMap<String, Value>),
}