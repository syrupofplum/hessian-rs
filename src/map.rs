use std::collections::HashMap;
use serde::{Serialize, Serializer};
use crate::value::Value;

pub enum Map {
    TypedMap(HashMap<Value, Value>),
    UntypedMap(HashMap<Value, Value>),
}

impl Serialize for Map {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        todo!()
    }
}
