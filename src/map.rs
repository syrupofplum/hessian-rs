use crate::value::Value;
use serde::{Serialize, Serializer};
use std::collections::HashMap;

pub enum Map {
    TypedMap(HashMap<Value, Value>),
    UntypedMap(HashMap<Value, Value>),
}

impl Serialize for Map {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        todo!()
    }
}
