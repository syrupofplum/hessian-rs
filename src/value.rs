use serde::ser::{Serialize};
use serde::Serializer;
use crate::binary::Hessian2Binary;
use crate::list::Hessian2List;
use crate::map::Hessian2Map;
use crate::error::Error;

pub enum Value {
    Binary(Hessian2Binary),
    Boolean(bool),
    Date(i64),
    Double(f64),
    Int(i32),
    List(Hessian2List),
    Long(i64),
    Map(Hessian2Map),
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
