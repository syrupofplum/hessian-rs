use serde::ser::{Serialize};
use serde::Serializer;
use crate::binary::Binary;
use crate::class::Class;
use crate::list::List;
use crate::map::Map;
use crate::error::Error;

pub enum Value<T> {
    Binary(Binary),
    Boolean(bool),
    Date(i64),
    Double(f64),
    Int(i32),
    List(List<T>),
    Long(i64),
    Map(Map),
    Null,
    Object(Class<T>),
    Ref,
    String(String),
    Type,
    TypeReferences,
}

impl<T> Serialize for Value<T>
where
    T: Serialize
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match self {
            Value::Binary(v) => {todo!()}
            Value::Boolean(v) => serializer.serialize_bool(*v),
            Value::Date(_) => {todo!()},
            Value::Double(v) => serializer.serialize_f64(*v),
            Value::Int(v) => serializer.serialize_i32(*v),
            Value::List(v) => v.serialize(serializer),
            Value::Long(v) => serializer.serialize_i64(*v),
            Value::Map(v) => v.serialize(serializer),
            Value::Null => serializer.serialize_none(),
            Value::Object(v) => v.serialize(serializer),
            Value::Ref => {todo!()}
            Value::String(v) => serializer.serialize_str(v),
            Value::Type => {todo!()}
            Value::TypeReferences => {todo!()}
        }
    }
}
