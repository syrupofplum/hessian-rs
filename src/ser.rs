use std::io;
use std::ops::Deref;
use serde::{ser, Serialize, Serializer as OtherSerializer};
use serde::ser::{SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant};
use bytes::{BytesMut, BufMut, Bytes};
use serde_json::ser::Compound;

use crate::constants::{BINARY_CHUNK_SIZE, STRING_CHUNK_SIZE, PRIMITIVE_TYPE_MAP, PrimitiveType};
use crate::error::{Error, Result};
use crate::list::get_primitive_type_str;

const I32_MAX_U32: u32 = i32::MAX as u32;
const I32_MIN_I64: i64 = i32::MIN as i64;
const I32_MAX_I64: i64 = i32::MAX as i64;
const I32_MAX_U64: u64 = i32::MAX as u64;
const I64_MAX_U64: u64 = i64::MAX as u64;
const F32_MIN_F64: f64 = f32::MIN as f64;
const F32_MAX_F64: f64 = f32::MAX as f64;

pub type BytesBuf = BytesMut;

#[doc(hidden)]
#[derive(Eq, PartialEq)]
pub enum State {
    Empty,
    First,
    Rest,
}

pub struct SerializeElementInfo<W> {
    type_name: String,
    value: Serializer<W>,
}

pub struct SerializeResult<'a, W> {
    ser: &'a mut Serializer<W>,
    state: State,
    value_result_list: Option<Vec<SerializeElementInfo<W>>>
}

impl<'a, W> SerializeResult<'a, W> {
    pub fn new(ser: &'a mut Serializer<W>, state: State) -> Self {
        SerializeResult {
            ser,
            state,
            value_result_list: None,
        }
    }
}

impl<'a, W> SerializeSeq for SerializeResult<'a, W>
where
    W: io::Write
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<Self::Ok> where T: Serialize {
        if State::Empty == self.state || State::First == self.state {
            let type_name = std::any::type_name::<T>();
            let ref mut ref_ser = *self.ser;
            if let Some(primitive_type) = PRIMITIVE_TYPE_MAP.get(type_name) {
                ref_ser.serialize_str(get_primitive_type_str(primitive_type))?;
            }
        }
        self.state = State::Rest;
        let ref mut ref_ser = *self.ser;
        value.serialize(ref_ser)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a, W> SerializeTuple for SerializeResult<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<Self::Ok> where T: Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl<'a, W> SerializeTupleStruct for SerializeResult<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<Self::Ok> where T: Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl<'a, W> SerializeTupleVariant for SerializeResult<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<Self::Ok> where T: Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl<'a, W> SerializeMap for SerializeResult<'a, W> {
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

impl<'a, W> SerializeStruct for SerializeResult<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok> where T: Serialize {
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a, W> SerializeStructVariant for SerializeResult<'a, W> {
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
    W: io::Write
{
    pub fn new(writer: W) -> Self {
        Serializer {
            writer
        }
    }

    fn write_buf(&mut self, bytes_buf: BytesMut) -> Result<()> {
        self.writer.write_all(bytes_buf.freeze().deref()).map_err(|_| Error{})
    }
}

impl <'a, W> ser::Serializer for &'a mut Serializer<W>
where
    W: io::Write
{
    type Ok = ();
    type Error = Error;
    type SerializeSeq = SerializeResult<'a, W>;
    type SerializeTuple = SerializeResult<'a, W>;
    type SerializeTupleStruct = SerializeResult<'a, W>;
    type SerializeTupleVariant = SerializeResult<'a, W>;
    type SerializeMap = SerializeResult<'a, W>;
    type SerializeStruct = SerializeResult<'a, W>;
    type SerializeStructVariant = SerializeResult<'a, W>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        let mut bytes_buf = BytesBuf::with_capacity(1);
        Formatter::format_bool(v, &mut bytes_buf)?;
        self.write_buf(bytes_buf)?;
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        let mut bytes_buf = BytesBuf::with_capacity(2);
        Formatter::format_i8(v, &mut bytes_buf)?;
        self.write_buf(bytes_buf)?;
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        let mut bytes_buf = BytesBuf::with_capacity(3);
        Formatter::format_i16(v, &mut bytes_buf)?;
        self.write_buf(bytes_buf)?;
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        let mut bytes_buf = BytesBuf::with_capacity(5);
        Formatter::format_i32(v, &mut bytes_buf)?;
        self.write_buf(bytes_buf)?;
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        let mut bytes_buf = BytesBuf::with_capacity(9);
        Formatter::format_i64(v, &mut bytes_buf)?;
        self.write_buf(bytes_buf)?;
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        let mut bytes_buf = BytesBuf::with_capacity(3);
        Formatter::format_u8(v, &mut bytes_buf)?;
        self.write_buf(bytes_buf)?;
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        let mut bytes_buf = BytesBuf::with_capacity(5);
        Formatter::format_u16(v, &mut bytes_buf)?;
        self.write_buf(bytes_buf)?;
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        let mut bytes_buf = BytesBuf::with_capacity(9);
        Formatter::format_u32(v, &mut bytes_buf)?;
        self.write_buf(bytes_buf)?;
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        let mut bytes_buf = BytesBuf::with_capacity(9);
        Formatter::format_u64(v, &mut bytes_buf)?;
        self.write_buf(bytes_buf)?;
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        let mut bytes_buf = BytesBuf::with_capacity(9);
        Formatter::format_f32(v, &mut bytes_buf)?;
        self.write_buf(bytes_buf)?;
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        let mut bytes_buf = BytesBuf::with_capacity(9);
        Formatter::format_f64(v, &mut bytes_buf)?;
        self.write_buf(bytes_buf)?;
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        let mut bytes_buf = BytesBuf::with_capacity(2);
        Formatter::format_char(v, &mut bytes_buf)?;
        self.write_buf(bytes_buf)?;
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        let v_chars_len = v.chars().count();
        let v_len = v.len();
        let mut bytes_buf = BytesBuf::with_capacity(if v_chars_len < STRING_CHUNK_SIZE {v_len + 3} else {(v_chars_len / STRING_CHUNK_SIZE + 1) * 3 + v_len});
        Formatter::format_str(v, &mut bytes_buf)?;
        self.write_buf(bytes_buf)?;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        let v_len = v.len();
        let mut bytes_buf = BytesBuf::with_capacity(if v_len < BINARY_CHUNK_SIZE {v_len + 3} else {(v_len / BINARY_CHUNK_SIZE + 1) * 3 + v_len});
        Formatter::format_binary(v, &mut bytes_buf)?;
        self.write_buf(bytes_buf)?;
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        let mut bytes_buf = BytesBuf::with_capacity(1);
        Formatter::format_none(&mut bytes_buf)?;
        self.writer.write_all(bytes_buf.freeze().deref()).map_err(|_| Error{})?;
        Ok(())
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
        let mut bytes_buf = BytesBuf::with_capacity(5);
        if let Some(v_len) = len {
            if 0 == v_len {
                Formatter::format_untyped_list_header(v_len, &mut bytes_buf)?;
            } else {
                Formatter::format_typed_list_header(v_len, &mut bytes_buf)?;
            }
            self.writer.write_all(bytes_buf.freeze().deref()).map_err(|_| Error{})?;
            return if v_len == 0 {
                Ok(SerializeResult::new(self, State::Empty))
            } else {
                Ok(SerializeResult::new(self, State::First))
            }
        }
        Err(Error{})
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
        let mut bytes_buf = BytesBuf::with_capacity(1 + name.len());
        Formatter::format_object_class_header(name, len, &mut bytes_buf)?;
        self.writer.write_all(bytes_buf.freeze().deref()).map_err(|_| Error{})?;
        return if len == 0 {
            Ok(SerializeResult::new(self, State::Empty))
        } else {
            Ok(SerializeResult::new(self, State::First))
        }
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
            // T
            true => buf.put_u8(0x54),
            // F
            false => buf.put_u8(0x46),
        }
        Ok(())
    }

    pub fn format_i8(v: i8, buf: &mut BytesBuf) -> Result<()> {
        Self::format_int_signed(v.into(), buf)
    }

    pub fn format_i16(v: i16, buf: &mut BytesBuf) -> Result<()> {
        Self::format_int_signed(v.into(), buf)
    }

    pub fn format_i32(v: i32, buf: &mut BytesBuf) -> Result<()> {
        Self::format_int_signed(v.into(), buf)
    }

    pub fn format_i64(v: i64, buf: &mut BytesBuf) -> Result<()> {
        Self::format_long_signed(v.into(), buf)
    }

    fn format_int_signed(v: i32, buf: &mut BytesBuf) -> Result<()> {
        match v {
            -16..=47 => {
                buf.put_u8((v + 0x90) as u8);
            },
            -2_048..=2_047 => {
                buf.put_u8((((v >> 8) & 0xff) + 0xc8) as u8);
                buf.put_u8((v & 0xff) as u8);
            },
            -262_144..=262_143 => {
                buf.put_u8((((v >> 16) & 0xff) + 0xd4) as u8);
                buf.put_u8(((v >> 8) & 0xff) as u8);
                buf.put_u8((v & 0xff) as u8);
            },
            _ => {
                // I
                buf.put_u8(0x49);
                buf.put_i32(v);
            },
        };
        Ok(())
    }

    fn format_long_signed(v: i64, buf: &mut BytesBuf) -> Result<()> {
        match v {
            -8..=15 => {
                buf.put_u8((v + 0xe0) as u8);
            },
            -2_048..=2_047 => {
                buf.put_u8((((v >> 8) & 0xff) + 0xf8) as u8);
                buf.put_u8((v & 0xff) as u8);
            },
            -262_144..=262_143 => {
                buf.put_u8((((v >> 16) & 0xff) + 0x3c) as u8);
                buf.put_u8(((v >> 8) & 0xff) as u8);
                buf.put_u8((v & 0xff) as u8);
            },
            I32_MIN_I64..=I32_MAX_I64 => {
                // Y
                buf.put_u8(0x59);
                buf.put_i32(v as i32);
            },
            _ => {
                // L
                buf.put_u8(0x4c);
                buf.put_i64(v);
            },
        };
        Ok(())
    }

    pub fn format_u8(v: u8, buf: &mut BytesBuf) -> Result<()> {
        Self::format_int_signed(v.into(), buf)
    }

    pub fn format_u16(v: u16, buf: &mut BytesBuf) -> Result<()> {
        Self::format_int_signed(v.into(), buf)
    }

    pub fn format_u32(v: u32, buf: &mut BytesBuf) -> Result<()> {
        if v <= I32_MAX_U32 {
            Self::format_int_signed(v as i32, buf)
        } else {
            Self::format_long_signed(v.into(), buf)
        }
    }

    pub fn format_u64(v: u64, buf: &mut BytesBuf) -> Result<()> {
        if v > I64_MAX_U64 {
            return Err(Error{});
        }
        Self::format_long_signed(v as i64, buf)
    }

    fn format_double(v: f64, buf: &mut BytesBuf) -> Result<()> {
        let v_trunc = v.trunc() as i32;
        let mut is_formatted = false;
        if v_trunc as f64 == v {
            match v_trunc {
                0 => {
                    buf.put_u8(0x5b);
                    is_formatted = true;
                },
                1 => {
                    buf.put_u8(0x5c);
                    is_formatted = true;
                },
                -128..=127 => {
                    buf.put_u8(0x5d);
                    buf.put_i8(v_trunc as i8);
                    is_formatted = true;
                },
                -32_768..=32_767 => {
                    buf.put_u8(0x5e);
                    buf.put_i16(v_trunc as i16);
                    is_formatted = true;
                },
                _ => {},
            }
        }
        if !is_formatted {
            // D
            buf.put_u8(0x44);
            buf.put_f64(v);
        }
        Ok(())
    }

    pub fn format_f32(v: f32, buf: &mut BytesBuf) -> Result<()> {
        Self::format_double(v.into(), buf)
    }

    pub fn format_f64(v: f64, buf: &mut BytesBuf) -> Result<()> {
        Self::format_double(v.into(), buf)
    }

    pub fn format_none(buf: &mut BytesBuf) -> Result<()> {
        // N
        buf.put_u8(0x4e);
        Ok(())
    }

    pub fn format_char(v: char, buf: &mut BytesBuf) -> Result<()> {
        Self::format_str(v.to_string().as_str(), buf)
    }

    pub fn format_str(v: &str, buf: &mut BytesBuf) -> Result<()> {
        let v_len = v.chars().count();
        if v_len < 32 {
            buf.put_u8(v_len as u8);
            buf.put(v.as_bytes());
        } else if v_len < 1_024 {
            buf.put_u8((((v_len >> 8) & 0xff) + 0x30) as u8);
            buf.put_u8((v_len & 0xff) as u8);
            buf.put(v.as_bytes());
        } else if v_len < STRING_CHUNK_SIZE {
            // S
            buf.put_u8(0x53);
            buf.put_u8(((v_len >> 8) & 0xff) as u8);
            buf.put_u8((v_len & 0xff) as u8);
            buf.put(v.as_bytes());
        } else {
            // R
            buf.put_u8(0x52);
            buf.put_u8(((STRING_CHUNK_SIZE >> 8) & 0xff) as u8);
            buf.put_u8((STRING_CHUNK_SIZE & 0xff) as u8);
            let mut ch_count = 0;
            let mut sum_utf8_len = 0;
            for ch in v.chars() {
                if ch_count >= STRING_CHUNK_SIZE {
                    break;
                }
                sum_utf8_len += ch.len_utf8();
                ch_count += 1;
            }
            buf.put((&v[..sum_utf8_len]).as_bytes());
            return Self::format_str(&v[sum_utf8_len..], buf);
        }
        Ok(())
    }

    pub fn format_binary(v: &[u8], buf: &mut BytesBuf) -> Result<()> {
        let v_len = v.len();
        if v_len < 16 {
            buf.put_u8((v_len + 0x20) as u8);
            buf.put(v);
        } else if v_len < 1024 {
            buf.put_u8((((v_len >> 8) & 0xff) + 0x34) as u8);
            buf.put_u8((v_len & 0xff) as u8);
            buf.put(v);
        } else if v_len <= BINARY_CHUNK_SIZE {
            // B
            buf.put_u8(0x42);
            buf.put_u8(((v_len >> 8) & 0xff) as u8);
            buf.put_u8((v_len & 0xff) as u8);
            buf.put(v);
        } else {
            // A
            buf.put_u8(0x41);
            buf.put_u8(((BINARY_CHUNK_SIZE >> 8) & 0xff) as u8);
            buf.put_u8((BINARY_CHUNK_SIZE & 0xff) as u8);
            buf.put(&v[..BINARY_CHUNK_SIZE]);
            return Self::format_binary(&v[BINARY_CHUNK_SIZE..], buf);
        }
        Ok(())
    }

    pub fn format_typed_list_header(v_len: usize, buf: &mut BytesBuf) -> Result<()> {
        if v_len > I32_MAX_U32 as usize {
            return Err(Error{});
        }
        if v_len < 8 {
            buf.put_u8((v_len + 0x70) as u8);
        } else {
            // V
            buf.put_u8(0x56);
            buf.put_i32(v_len as i32);
        }
        Ok(())
    }

    pub fn format_untyped_list_header(v_len: usize, buf: &mut BytesBuf) -> Result<()> {
        if v_len > I32_MAX_U32 as usize {
            return Err(Error{});
        }
        if v_len < 8 {
            buf.put_u8((v_len + 0x78) as u8);
        } else {
            // X
            buf.put_u8(0x58);
            buf.put_i32(v_len as i32);
        }
        Ok(())
    }

    pub fn format_object_class_header(name: &str, len: usize, buf: &mut BytesBuf) -> Result<()> {
        // C
        buf.put_u8(0x43);
        Formatter::format_str(name, buf)?;
        Formatter::format_int_signed(len as i32, buf)?;
        Ok(())
    }
}
