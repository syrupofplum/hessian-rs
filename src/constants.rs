use std::hash::Hasher;
use phf::phf_map;

pub const STRING_CHUNK_SIZE: usize = 32_768;
pub const BINARY_CHUNK_SIZE: usize = 8_189;

#[derive(Clone)]
pub enum PrimitiveType {
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    Boolean,
    Char,
}

pub static PRIMITIVE_TYPE_MAP: phf::Map<&'static str, PrimitiveType> = phf_map! {
    "u8" => PrimitiveType::Int,
    "u16" => PrimitiveType::Int,
    "u32" => PrimitiveType::Long,
    "u64" => PrimitiveType::Long,
    "i8" => PrimitiveType::Int,
    "i16" => PrimitiveType::Int,
    "i32" => PrimitiveType::Int,
    "i64" => PrimitiveType::Long,
};

impl TryFrom<u8> for PrimitiveType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == PrimitiveType::Byte as u8 => Ok(PrimitiveType::Byte),
            x if x == PrimitiveType::Short as u8 => Ok(PrimitiveType::Short),
            x if x == PrimitiveType::Int as u8 => Ok(PrimitiveType::Int),
            x if x == PrimitiveType::Long as u8 => Ok(PrimitiveType::Long),
            x if x == PrimitiveType::Float as u8 => Ok(PrimitiveType::Float),
            x if x == PrimitiveType::Double as u8 => Ok(PrimitiveType::Double),
            x if x == PrimitiveType::Boolean as u8 => Ok(PrimitiveType::Boolean),
            x if x == PrimitiveType::Char as u8 => Ok(PrimitiveType::Char),
            _ => Err(())
        }
    }
}
