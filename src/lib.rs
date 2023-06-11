use crate::ser::Serializer;
use serde::Serialize;

pub mod binary;
pub mod class;
pub mod constants;
pub mod error;
pub mod list;
pub mod map;
pub mod ser;
pub mod value;

pub fn to_hessian2<T>(obj: &value::Value<T>) -> error::Result<Vec<u8>>
where
    T: Serialize,
{
    let mut writer = Vec::with_capacity(128);
    let mut ser = Serializer::new(&mut writer);
    obj.serialize(&mut ser)?;
    Ok(writer)
}
