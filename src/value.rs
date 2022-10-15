use serde::ser::{Serialize};
use serde::Serializer;
use crate::binary::BinaryData;
use crate::list::List;
use crate::map::Map;
use crate::error::Error;

pub enum Value {
    Binary(BinaryData),
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
        todo!()
    }
}
