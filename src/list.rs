use serde::{Serialize, Serializer};
use crate::constants::PrimitiveType;
use crate::value::Value;

pub enum List<T> {
    TypedList(String, Vec<Value<T>>),
    UntypedList(Vec<Value<T>>),
}

pub fn get_primitive_type_str(primitive_type: &PrimitiveType) -> &'static str {
    return match primitive_type {
        PrimitiveType::Byte => "[byte",
        PrimitiveType::Short => "[short",
        PrimitiveType::Int => "[int",
        PrimitiveType::Long => "[long",
        PrimitiveType::Float => "[float",
        PrimitiveType::Double => "[double",
        PrimitiveType::Boolean => "[boolean",
        PrimitiveType::Char => "[char",
    }
}

impl<T> Serialize for List<T>
where
    T: Serialize
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match self {
            List::TypedList(m_type, v_list) => {
                todo!()
            },
            List::UntypedList(v_list) => {
                todo!()
            },
        }

    }
}
