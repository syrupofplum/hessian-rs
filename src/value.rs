use crate::binary::Binary;
use crate::class::Class;
use crate::list::List;
use crate::map::Map;
use serde::ser::Serialize;
use serde::Serializer;

pub enum Value<T = ()> {
    Binary(Binary),
    Boolean(bool),
    Date(i64),
    Double(f64),
    Int(i32),
    List(List),
    Long(i64),
    Map(Map),
    Null,
    Ref,
    String(String),
    Type,
    TypeReferences,
    Object(Class<T>),
}

impl<T> Serialize for Value<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Value::Binary(_v) => {
                todo!()
            }
            Value::Boolean(v) => serializer.serialize_bool(*v),
            Value::Date(_) => {
                todo!()
            }
            Value::Double(v) => serializer.serialize_f64(*v),
            Value::Int(v) => serializer.serialize_i32(*v),
            Value::List(v) => v.serialize(serializer),
            Value::Long(v) => serializer.serialize_i64(*v),
            Value::Map(v) => v.serialize(serializer),
            Value::Null => serializer.serialize_none(),
            Value::Ref => {
                todo!()
            }
            Value::String(v) => serializer.serialize_str(v),
            Value::Type => {
                todo!()
            }
            Value::TypeReferences => {
                todo!()
            }
            Value::Object(v) => v.serialize(serializer),
        }
    }
}
