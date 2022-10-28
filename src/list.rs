use serde::{Serialize, Serializer};
use crate::constants::PrimitivesType;
use crate::value::Value;

pub enum List {
    TypedList(String, Vec<Value>),
    UntypedList(Vec<Value>),
}

pub fn get_primitive_type_str(primitive_type: PrimitivesType) -> &'static str {
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
            List::TypedList(m_type, v_list) => {
                let v_len = v_list.len();
                if v_len < 8 {
                    // serializer.serialize_u8(0x70 + v_len as u8)?;
                    // serializer.serialize_str(m_type)?;
                    // for v in v_list {
                    //     v.serialize(serializer);
                    // }
                    return serializer.collect_seq(v_list);
                } else {
                    return serializer.serialize_i32(0);
                }
            },
            List::UntypedList(v_list) => {
                return serializer.serialize_i32(0);
            },
        }

    }
}
