use serde::{Serialize, Serializer};

pub enum Map {
    TypedMap,
    UntypedMap,
}

impl Serialize for Map {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        todo!()
    }
}
