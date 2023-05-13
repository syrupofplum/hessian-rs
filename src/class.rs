use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use std::io;

pub struct Class<T> {
    class_path: &'static str,
    data: T,
}

impl<T> Class<T> {
    pub fn new(class_path: &'static str, data: T) -> Self {
        Class { class_path, data }
    }
}

impl<T> serde::Serialize for Class<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct(self.class_path, usize::MAX)?;
        s.serialize_field("", &self.data)?;
        s.end()
    }
}
