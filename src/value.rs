use serde::ser::{Serialize};
use serde::Serializer;
use crate::binary::Binary;
use crate::list::List;
use crate::map::Map;
use crate::error::Error;

pub enum Value {
    Binary(Binary),
    Boolean(bool),
    Date(i64),
    Double(f64),
    Int(i32),
    List(List),
    Long(i64),
    Map(Map),
    Null,
    Object,
    Ref,
    String(String),
    Type,
    TypeReferences,
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match self {
            Value::Binary(_) => {}
            Value::Boolean(_) => {}
            Value::Date(_) => {}
            Value::Double(_) => {}
            Value::Int(v) => {
                return serializer.serialize_i32(*v);
            },
            Value::List(_) => {}
            Value::Long(_) => {}
            Value::Map(_) => {}
            Value::Null => {}
            Value::Object => {}
            Value::Ref => {}
            Value::String(_) => {}
            Value::Type => {}
            Value::TypeReferences => {}
        }
        return serializer.serialize_i32(0);
    }
}
