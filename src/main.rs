use std::fmt::{Debug, Display};
use std::io;
use serde::{ser, Serialize};
use serde::ser::{SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant, StdError};
use bytes::{BytesMut, BufMut};

type BytesBuf = BytesMut;

pub struct Error {

}

impl StdError for Error {}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self where T: Display {
        todo!()
    }
}

pub type Result<T> = core::result::Result<T, Error>;

pub struct SerializeResult {

}

impl SerializeSeq for SerializeResult {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<Self::Ok> where T: Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl SerializeTuple for SerializeResult {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<Self::Ok> where T: Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl SerializeTupleStruct for SerializeResult {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<Self::Ok> where T: Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl SerializeTupleVariant for SerializeResult {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<Self::Ok> where T: Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl SerializeMap for SerializeResult {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<Self::Ok> where T: Serialize {
        todo!()
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<Self::Ok> where T: Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl SerializeStruct for SerializeResult {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok> where T: Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl SerializeStructVariant for SerializeResult {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok> where T: Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

pub struct Serializer<W> {
    writer: W
}

impl <W> Serializer<W>
where
    W: io::Write,
{
    pub fn new(writer: W) -> Self {
        Serializer {
            writer
        }
    }
}

impl <W> ser::Serializer for Serializer<W>
where
    W: io::Write
{
    type Ok = ();
    type Error = Error;
    type SerializeSeq = SerializeResult;
    type SerializeTuple = SerializeResult;
    type SerializeTupleStruct = SerializeResult;
    type SerializeTupleVariant = SerializeResult;
    type SerializeMap = SerializeResult;
    type SerializeStruct = SerializeResult;
    type SerializeStructVariant = SerializeResult;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok> where T: Serialize {
        todo!()
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> Result<Self::Ok> where T: Serialize {
        todo!()
    }

    fn serialize_newtype_variant<T: ?Sized>(self, name: &'static str, variant_index: u32, variant: &'static str, value: &T) -> Result<Self::Ok> where T: Serialize {
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        todo!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        todo!()
    }

    fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct> {
        todo!()
    }

    fn serialize_tuple_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeTupleVariant> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        todo!()
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        todo!()
    }

    fn serialize_struct_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeStructVariant> {
        todo!()
    }
}

pub struct Formatter {
}

impl Formatter {
    pub fn format_bool(v: bool, buf: &mut BytesBuf) -> Result<()> {
        match v {
            true => buf.put_u8(0x54),
            false => buf.put_u8(0x46),
        }
        Ok(())
    }

    pub fn format_i8(v: i8, buf: &mut BytesBuf) -> Result<()> {
        Formatter::format_i32(v.into(), buf)
    }

    pub fn format_i32(v: i32, buf: &mut BytesBuf) -> Result<()> {
        match v {
            -16..=47 => {
                buf.put_i8((v + 0x90) as i8);
            },
            -2048..=2047 => {

            },
            -262144..=262143 => {

            },
            _ => {

            },
        };
        Ok(())
    }
}

fn main() {
    use hessian_rs;
    let mut stdout = std::io::stdout();
    let mut ser: hessian_rs::ser::Serializer<std::io::Stdout> = hessian_rs::ser::Serializer::new(&mut stdout);
    ser.serialize_value(&hessian_rs::Value::Bool(true));
}
