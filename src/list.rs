use serde::{Serialize, Serializer};
use crate::constants::PrimitiveType;
use crate::value::Value;

pub enum Hessian2List {
    TypedList(String, Vec<Value>),
    UntypedList(Vec<Value>),
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

impl Serialize for Hessian2List {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match self {
            Hessian2List::TypedList(m_type, v_list) => {
                todo!()
            },
            Hessian2List::UntypedList(v_list) => {
                todo!()
            },
        }

    }
}
