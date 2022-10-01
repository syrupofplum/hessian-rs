use std::fmt::{Debug, Display};
use std::io;
use std::io::Write;
use std::ops::Deref;
use serde::{ser, Serialize, Serializer as OtherSerializer};
use serde::ser::{SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant, StdError};
use bytes::{BytesMut, BufMut, Bytes};

const I32_MIN_I64: i64 = i32::MIN as i64;
const I32_MAX_I64: i64 = i32::MAX as i64;
const I32_MAX_U64: u64 = i32::MAX as u64;
const F32_MIN_F64: f64 = f32::MIN as f64;
const F32_MAX_F64: f64 = f32::MAX as f64;

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

pub struct Serializer<'a, W> {
    writer: &'a mut W
}

impl <'a, W> Serializer<'a, W>
where
    W: io::Write,
{
    pub fn new(writer: &'a mut W) -> Self {
        Serializer {
            writer
        }
    }
}

impl <'a, W> ser::Serializer for &'a mut Serializer<'a, W>
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
        let mut bytes_buf = BytesBuf::with_capacity(1);
        Formatter::format_bool(v, &mut bytes_buf)?;
        self.writer.write_all(bytes_buf.freeze().deref()).map_err(|_| Error{})?;
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        let mut bytes_buf = BytesBuf::with_capacity(2);
        Formatter::format_i8(v, &mut bytes_buf)?;
        self.writer.write_all(bytes_buf.freeze().deref()).map_err(|_| Error{})?;
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        let mut bytes_buf = BytesBuf::with_capacity(3);
        Formatter::format_i16(v, &mut bytes_buf)?;
        self.writer.write_all(bytes_buf.freeze().deref()).map_err(|_| Error{})?;
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        let mut bytes_buf = BytesBuf::with_capacity(5);
        Formatter::format_i32(v, &mut bytes_buf)?;
        self.writer.write_all(bytes_buf.freeze().deref()).map_err(|_| Error{})?;
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        let mut bytes_buf = BytesBuf::with_capacity(9);
        Formatter::format_i64(v, &mut bytes_buf)?;
        self.writer.write_all(bytes_buf.freeze().deref()).map_err(|_| Error{})?;
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        let mut bytes_buf = BytesBuf::with_capacity(2);
        Formatter::format_u8(v, &mut bytes_buf)?;
        self.writer.write_all(bytes_buf.freeze().deref()).map_err(|_| Error{})?;
        Ok(())
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
        let mut bytes_buf = BytesBuf::with_capacity(9);
        Formatter::format_f32(v, &mut bytes_buf)?;
        self.writer.write_all(bytes_buf.freeze().deref()).map_err(|_| Error{})?;
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        let mut bytes_buf = BytesBuf::with_capacity(9);
        Formatter::format_f64(v, &mut bytes_buf)?;
        self.writer.write_all(bytes_buf.freeze().deref()).map_err(|_| Error{})?;
        Ok(())
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

    pub fn format_u64(v: u64, buf: &mut BytesBuf) -> Result<()> {
        if v > I32_MAX_U64 {
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
}

struct BytesBufWriter {
    bytes_buf: BytesBuf,
    bytes_result: Option<Bytes>
}

impl BytesBufWriter {
    fn new() -> Self {
        BytesBufWriter {
            bytes_buf: BytesBuf::new(),
            bytes_result: None,
        }
    }

    fn get(&self) -> Bytes {
        // println!("{:?}", self.bytes_result);
        self.bytes_result.clone().unwrap()
    }
}

impl io::Write for BytesBufWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.bytes_buf.put(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        let bytes_buf = self.bytes_buf.clone();
        let bytes = bytes_buf.freeze();
        self.bytes_result = Some(bytes);
        Ok(())
    }
}

mod tests {
    use std::io::Write;
    use std::ops::Deref;
    use serde::Serializer as OtherSerializer;
    use crate::{BytesBufWriter, Error, Result, Serializer};
    use hessian_rs;
    use j4rs::{ClasspathEntry, errors, Instance, InvocationArg, Jvm, JvmBuilder};

    #[test]
    fn get_jar_result() -> Result<()> {
        let entry = ClasspathEntry::new("./hessian-java.jar");
        let jvm = JvmBuilder::new()
            .classpath_entry(entry)
            .build()
            .map_err(|_| Error{})?;
        let bool_arg = InvocationArg::try_from(true).map_err(|_| Error{})?
            .into_primitive().map_err(|_| Error{})?;
        let java_result = jvm.invoke_static(
            "com.syrupofplum.hessian.Main",
            "serializeBool",
            &[bool_arg],
        ).map_err(|_| Error{})?;

        // todo

        Ok(())
    }

    #[test]
    fn test_bool() {
        const V: bool = true;

        let mut other_buf = BytesBufWriter::new();
        let mut other_ser = hessian_rs::ser::Serializer::new(&mut other_buf);
        other_ser.serialize_value(&hessian_rs::Value::Bool(V)).unwrap();
        other_buf.flush().unwrap();

        let mut buf = BytesBufWriter::new();
        let mut ser = Serializer::new(&mut buf);
        ser.serialize_bool(V).unwrap();
        buf.flush().unwrap();

        assert_eq!(other_buf.get(), buf.get());
    }

    #[test]
    fn test_int_1_octet() {
        const V: i8 = 35;

        let mut other_buf = BytesBufWriter::new();
        let mut other_ser = hessian_rs::ser::Serializer::new(&mut other_buf);
        other_ser.serialize_value(&hessian_rs::Value::Int(V as i32)).unwrap();
        other_buf.flush().unwrap();

        let mut buf = BytesBufWriter::new();
        let mut ser = Serializer::new(&mut buf);
        ser.serialize_i8(V).unwrap();
        buf.flush().unwrap();

        assert_eq!(other_buf.get(), buf.get());
    }

    #[test]
    fn test_int_2_octet() {
        const V: i16 = -1837;

        let mut other_buf = BytesBufWriter::new();
        let mut other_ser = hessian_rs::ser::Serializer::new(&mut other_buf);
        other_ser.serialize_value(&hessian_rs::Value::Int(V as i32)).unwrap();
        other_buf.flush().unwrap();

        let mut buf = BytesBufWriter::new();
        let mut ser = Serializer::new(&mut buf);
        ser.serialize_i16(V).unwrap();
        buf.flush().unwrap();

        assert_eq!(other_buf.get(), buf.get());
    }

    #[test]
    fn test_int_3_octet() {
        const V: i32 = 136289;

        let mut other_buf = BytesBufWriter::new();
        let mut other_ser = hessian_rs::ser::Serializer::new(&mut other_buf);
        other_ser.serialize_value(&hessian_rs::Value::Int(V as i32)).unwrap();
        other_buf.flush().unwrap();

        let mut buf = BytesBufWriter::new();
        let mut ser = Serializer::new(&mut buf);
        ser.serialize_i32(V).unwrap();
        buf.flush().unwrap();

        assert_eq!(other_buf.get(), buf.get());
    }

    #[test]
    fn test_int_4_octet() {
        const V: i32 = 9087387;

        let mut other_buf = BytesBufWriter::new();
        let mut other_ser = hessian_rs::ser::Serializer::new(&mut other_buf);
        other_ser.serialize_value(&hessian_rs::Value::Int(V as i32)).unwrap();
        other_buf.flush().unwrap();

        let mut buf = BytesBufWriter::new();
        let mut ser = Serializer::new(&mut buf);
        ser.serialize_i32(V).unwrap();
        buf.flush().unwrap();

        assert_eq!(other_buf.get(), buf.get());
    }

    #[test]
    fn test_int_8_octet() {
        const V: i64 = 298374982523759;

        let mut other_buf = BytesBufWriter::new();
        let mut other_ser = hessian_rs::ser::Serializer::new(&mut other_buf);
        other_ser.serialize_value(&hessian_rs::Value::Long(V as i64)).unwrap();
        other_buf.flush().unwrap();

        let mut buf = BytesBufWriter::new();
        let mut ser = Serializer::new(&mut buf);
        ser.serialize_i64(V).unwrap();
        buf.flush().unwrap();

        assert_eq!(other_buf.get(), buf.get());
    }

    #[test]
    fn test_double_8_octet() {
        const V: f64 = 897398.5747673;

        let mut other_buf = BytesBufWriter::new();
        let mut other_ser = hessian_rs::ser::Serializer::new(&mut other_buf);
        other_ser.serialize_value(&hessian_rs::Value::Double(V as f64)).unwrap();
        other_buf.flush().unwrap();

        let mut buf = BytesBufWriter::new();
        let mut ser = Serializer::new(&mut buf);
        ser.serialize_f64(V).unwrap();
        buf.flush().unwrap();

        println!("{:?}", buf.get().deref());

        // hessian_rs is wrong
        assert_eq!(other_buf.get(), buf.get());
    }
}

fn main() {
}
