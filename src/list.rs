use serde::{Serialize, Serializer};
use serde::ser::{SerializeSeq, SerializeTupleVariant};
use crate::constants::{PRIMITIVE_TYPE_MAP, PrimitiveType};
use crate::value::Value;

#[derive(Clone)]
pub enum TypedListType {
    PrimitiveType(PrimitiveType),
    CustomType(&'static str)
}

pub enum List<T> {
    TypedList(TypedListType, Vec<Value<T>>),
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
        PrimitiveType::String => "[string",
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
                let mut typed_list_type = m_type.clone();
                let type_name = std::any::type_name::<T>();
                if let Some(primitive_type) = PRIMITIVE_TYPE_MAP.get(type_name) {
                    typed_list_type = TypedListType::PrimitiveType(primitive_type.clone());
                }
                let seq_type = match typed_list_type {
                    TypedListType::PrimitiveType(primitive_type) => get_primitive_type_str(&primitive_type),
                    TypedListType::CustomType(custom_type) => custom_type,
                };
                let mut tv = serializer.serialize_tuple_variant("TypedList", v_list.len() as u32, seq_type, usize::MAX)?;
                tv.serialize_field(v_list)?;
                tv.end()
            },
            List::UntypedList(v_list) => {
                todo!()
            },
        }

    }
}
