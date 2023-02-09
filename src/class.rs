use std::io;
use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;

pub struct Class {
    class_path: &'static str,
    data: Box<dyn Serialize>
}

impl Class {
    pub fn new(class_path: &'static str, data: Box<dyn Serialize>) -> Self {
        Class {
            class_path,
            data,
        }
    }
}

impl serde::Serialize for Class
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut s = serializer.serialize_struct(self.class_path, usize::MAX)?;
        s.serialize_field("", &self.data)?;
        s.end()
    }
}
