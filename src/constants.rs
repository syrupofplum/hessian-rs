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
    String,
}

pub static PRIMITIVE_TYPE_MAP: phf::Map<&'static str, PrimitiveType> = phf_map! {
    "u8" => PrimitiveType::Int,
    "&u8" => PrimitiveType::Int,
    "u16" => PrimitiveType::Int,
    "&u16" => PrimitiveType::Int,
    "u32" => PrimitiveType::Long,
    "&u32" => PrimitiveType::Long,
    "u64" => PrimitiveType::Long,
    "&u64" => PrimitiveType::Long,
    "i8" => PrimitiveType::Int,
    "&i8" => PrimitiveType::Int,
    "i16" => PrimitiveType::Int,
    "&i16" => PrimitiveType::Int,
    "i32" => PrimitiveType::Int,
    "&i32" => PrimitiveType::Int,
    "i64" => PrimitiveType::Long,
    "&i64" => PrimitiveType::Long,
    "alloc::string::String" => PrimitiveType::String,
    "&str" => PrimitiveType::String,
};
