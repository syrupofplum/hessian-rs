use std::hash::Hasher;

pub const STRING_CHUNK_SIZE: usize = 32_768;
pub const BINARY_CHUNK_SIZE: usize = 8_189;

pub enum PrimitivesType {
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    Boolean,
    Char,
}

impl TryFrom<u8> for PrimitivesType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == PrimitivesType::Byte as u8 => Ok(PrimitivesType::Byte),
            x if x == PrimitivesType::Short as u8 => Ok(PrimitivesType::Short),
            x if x == PrimitivesType::Int as u8 => Ok(PrimitivesType::Int),
            x if x == PrimitivesType::Long as u8 => Ok(PrimitivesType::Long),
            x if x == PrimitivesType::Float as u8 => Ok(PrimitivesType::Float),
            x if x == PrimitivesType::Double as u8 => Ok(PrimitivesType::Double),
            x if x == PrimitivesType::Boolean as u8 => Ok(PrimitivesType::Boolean),
            x if x == PrimitivesType::Char as u8 => Ok(PrimitivesType::Char),
            _ => Err(())
        }
    }
}
