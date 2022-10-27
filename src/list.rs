use serde::{Serialize, Serializer};
use crate::constants::PrimitivesType;
use crate::value::Value;

pub enum List {
    TypedList(String, Vec<Value>),
    UntypedList(Vec<Value>),
}

fn get_primitive_type_str(primitive_type: PrimitivesType) -> &'static str {
    return match primitive_type {
        PrimitivesType::Byte => "[byte",
        PrimitivesType::Short => "[short",
        PrimitivesType::Int => "[int",
        PrimitivesType::Long => "[long",
        PrimitivesType::Float => "[float",
        PrimitivesType::Double => "[double",
        PrimitivesType::Boolean => "[boolean",
        PrimitivesType::Char => "[char",
    }
}

impl Serialize for List {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match self {
            List::TypedList(m_type,value_list) => {},
            List::UntypedList(value_list) => {},
        }
        Ok(())
    }
}
