use crate::constants::{PrimitiveType, PRIMITIVE_TYPE_MAP};
use crate::value::Value;
use serde::ser::{SerializeSeq, SerializeTupleVariant};
use serde::{Serialize, Serializer};

#[derive(Clone)]
pub enum TypedListType {
    PrimitiveType(PrimitiveType),
    CustomType(&'static str),
}

pub enum List {
    TypedList(TypedListType, Vec<Value>),
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
        PrimitiveType::String => "[string",
    };
}

impl List {
    fn get_typed_list_type(m_type: &TypedListType) -> TypedListType {
        let typed_list_type = m_type.clone();
        typed_list_type
    }

    fn get_seq_type(typed_list_type: TypedListType) -> &'static str {
        match typed_list_type {
            TypedListType::PrimitiveType(primitive_type) => get_primitive_type_str(&primitive_type),
            TypedListType::CustomType(custom_type) => custom_type,
        }
    }
}

impl Serialize for List {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            List::TypedList(m_type, v_list) => {
                let typed_list_type = List::get_typed_list_type(m_type);
                let seq_type = List::get_seq_type(typed_list_type);
                let mut tv = serializer.serialize_tuple_variant(
                    "TypedList",
                    v_list.len() as u32,
                    seq_type,
                    usize::MAX,
                )?;
                tv.serialize_field(v_list)?;
                tv.end()
            }
            List::UntypedList(v_list) => {
                let mut tv = serializer.serialize_tuple_variant(
                    "UntypedList",
                    v_list.len() as u32,
                    "",
                    usize::MAX,
                )?;
                tv.serialize_field(v_list)?;
                tv.end()
            }
        }
    }
}
