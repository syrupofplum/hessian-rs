use std::io;
use std::io::Write;
use std::ops::Deref;
use serde::Serializer as OtherSerializer;
use bytes::{BytesMut, BufMut, Bytes};
use serde::ser::SerializeSeq;
use hessian_rs::ser::{Serializer, BytesBuf};
use hessian_rs::error::{Error, Result};
use hessian_rs::list::{get_primitive_type_str, List};
use hessian_rs::value::Value;
use hessian_rs::constants::PrimitiveType;

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

impl Write for BytesBufWriter {
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

#[test]
fn test_bool_false() {
    const V: bool = false;

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_bool(V).unwrap();
    buf.flush().unwrap();

    assert_eq!([0x46], buf.get().deref());
}

#[test]
fn test_bool_true() {
    const V: bool = true;

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_bool(V).unwrap();
    buf.flush().unwrap();

    assert_eq!([0x54], buf.get().deref());
}

#[test]
fn test_int_1_1() {
    const V: i8 = 35;

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_i8(V).unwrap();
    buf.flush().unwrap();

    assert_eq!([0xb3], buf.get().deref());
}

#[test]
fn test_int_1_2() {
    const V: i8 = -11;

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_i8(V).unwrap();
    buf.flush().unwrap();

    assert_eq!([0x85], buf.get().deref());
}

#[test]
fn test_int_2_1() {
    const V: i16 = 117;

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_i16(V).unwrap();
    buf.flush().unwrap();

    assert_eq!([0xc8,0x75], buf.get().deref());
}

#[test]
fn test_int_2_2() {
    const V: i16 = 1736;

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_i16(V).unwrap();
    buf.flush().unwrap();

    assert_eq!([0xce,0xc8], buf.get().deref());
}

#[test]
fn test_int_2_3() {
    const V: i16 = -1837;

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_i16(V).unwrap();
    buf.flush().unwrap();

    assert_eq!([0xc0,0xd3], buf.get().deref());
}

#[test]
fn test_int_3_1() {
    const V: i32 = 136289;

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_i32(V).unwrap();
    buf.flush().unwrap();

    assert_eq!([0xd6,0x14,0x61], buf.get().deref());
}

#[test]
fn test_int_3_2() {
    const V: i32 = -3683;

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_i32(V).unwrap();
    buf.flush().unwrap();

    assert_eq!([0xd3,0xf1,0x9d], buf.get().deref());
}

#[test]
fn test_int_3_3() {
    const V: i32 = -87236;

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_i32(V).unwrap();
    buf.flush().unwrap();

    assert_eq!([0xd2,0xab,0x3c], buf.get().deref());
}

#[test]
fn test_int_5_1() {
    const V: i32 = 9087387;

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_i32(V).unwrap();
    buf.flush().unwrap();

    assert_eq!([0x49,0x0,0x8a,0xa9,0x9b], buf.get().deref());
}

#[test]
fn test_int_5_2() {
    const V: i32 = -7236247;

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_i32(V).unwrap();
    buf.flush().unwrap();

    assert_eq!([0x49,0xff,0x91,0x95,0x69], buf.get().deref());
}

#[test]
fn test_int_5_3() {
    const V: u32 = 2147483647;

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_u32(V).unwrap();
    buf.flush().unwrap();

    assert_eq!([0x49,0x7f,0xff,0xff,0xff], buf.get().deref());
}

#[test]
fn test_long_1_1() {
    const V: i64 = 7;

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_i64(V).unwrap();
    buf.flush().unwrap();

    assert_eq!([0xe7], buf.get().deref());
}

#[test]
fn test_long_1_2() {
    const V: u64 = 15;

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_u64(V).unwrap();
    buf.flush().unwrap();

    assert_eq!([0xef], buf.get().deref());
}

#[test]
fn test_long_1_3() {
    const V: i64 = -8;

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_i64(V).unwrap();
    buf.flush().unwrap();

    assert_eq!([0xd8], buf.get().deref());
}

#[test]
fn test_long_1_4() {
    const V: i64 = 0;

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_i64(V).unwrap();
    buf.flush().unwrap();

    assert_eq!([0xe0], buf.get().deref());
}

#[test]
fn test_long_2_1() {
    const V: i64 = 2047;

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_i64(V).unwrap();
    buf.flush().unwrap();

    assert_eq!([0xff,0xff], buf.get().deref());
}

#[test]
fn test_long_2_2() {
    const V: i64 = -2048;

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_i64(V).unwrap();
    buf.flush().unwrap();

    assert_eq!([0xf0,0x0], buf.get().deref());
}

#[test]
fn test_long_2_3() {
    const V: u64 = 1386;

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_u64(V).unwrap();
    buf.flush().unwrap();

    assert_eq!([0xfd,0x6a], buf.get().deref());
}

#[test]
fn test_long_3_1() {
    const V: u64 = 36263;

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_u64(V).unwrap();
    buf.flush().unwrap();

    assert_eq!([0x3c,0x8d,0xa7], buf.get().deref());
}

#[test]
fn test_long_5_1() {
    const V: u64 = 254132517;

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_u64(V).unwrap();
    buf.flush().unwrap();

    assert_eq!([0x59,0xf,0x25,0xc1,0x25], buf.get().deref());
}

#[test]
fn test_long_9_1() {
    const V: i64 = 298374982523759;

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_i64(V).unwrap();
    buf.flush().unwrap();

    assert_eq!([0x4c,0x0,0x1,0xf,0x5e,0xd6,0xd7,0xdb,0x6f], buf.get().deref());
}

#[test]
fn test_long_9_2() {
    const V: i64 = 2147483648;

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_i64(V).unwrap();
    buf.flush().unwrap();

    assert_eq!([0x4c,0x0,0x0,0x0,0x0,0x80,0x0,0x0,0x0], buf.get().deref());
}

#[test]
fn test_long_9_3() {
    const V: u64 = 9223372036854775807;

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_u64(V).unwrap();
    buf.flush().unwrap();

    assert_eq!([0x4c,0x7f,0xff,0xff,0xff,0xff,0xff,0xff,0xff], buf.get().deref());
}

#[test]
fn test_long_9_4() {
    // exceeded i64 max limit
    const V: u64 = 9223372036854775808;

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    assert!(ser.serialize_u64(V).is_err());
}

#[test]
fn test_double_9_1() {
    const V: f64 = 897398.5747673;

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_f64(V).unwrap();
    buf.flush().unwrap();

    // hessian_rs is wrong
    assert_eq!([0x44,0x41,0x2b,0x62,0xed,0x26,0x47,0xe6,0x49], buf.get().deref());
}

#[test]
fn test_string_1_1() {
    const V: &str = "";

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_str(V).unwrap();
    buf.flush().unwrap();

    assert_eq!([0x0], buf.get().deref());
}

#[test]
fn test_string_2_1() {
    const V: &str = "c";

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_str(V).unwrap();
    buf.flush().unwrap();

    assert_eq!([0x1,0x63], buf.get().deref());
}

#[test]
fn test_string_3_1() {
    let mut v_string: String = "".to_string();
    for _ in 0..32 {
        v_string.push('c');
    }
    let v: &str = v_string.as_str();

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_str(v).unwrap();
    buf.flush().unwrap();

    let mut ret_supposed: [u8; 34] = [0; 34];
    ret_supposed[0] = 0x30;
    ret_supposed[1] = 0x20;
    for i in 2..34 {
        ret_supposed[i] = 0x63;
    }
    assert_eq!(ret_supposed, buf.get().deref());
}

#[test]
fn test_string_4_1() {
    let mut v_string: String = "".to_string();
    for _ in 0..1_024 {
        v_string.push('c');
    }
    let v: &str = v_string.as_str();

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_str(v).unwrap();
    buf.flush().unwrap();

    let mut ret_supposed: [u8; 1_027] = [0; 1_027];
    ret_supposed[0] = 0x53;
    ret_supposed[1] = 0x04;
    ret_supposed[2] = 0x00;
    for i in 3..1_027 {
        ret_supposed[i] = 0x63;
    }
    assert_eq!(ret_supposed, buf.get().deref());
}

#[test]
fn test_string_5_1() {
    let mut v_string: String = "".to_string();
    for _ in 0..32_769 {
        v_string.push('c');
    }
    let v: &str = v_string.as_str();

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_str(v).unwrap();
    buf.flush().unwrap();

    let mut ret_supposed: [u8; 32_773] = [0; 32_773];
    ret_supposed[0] = 0x52;
    ret_supposed[1] = 0x80;
    ret_supposed[2] = 0x00;
    for i in 3..32_771 {
        ret_supposed[i] = 0x63;
    }
    ret_supposed[32_771] = 0x1;
    ret_supposed[32_772] = 0x63;
    assert_eq!(ret_supposed, buf.get().deref());
}

#[test]
fn test_string_5_2() {
    let mut v_string: String = "".to_string();
    for _ in 0..32_769 {
        v_string.push('这');
    }
    let v: &str = v_string.as_str();

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_str(v).unwrap();
    buf.flush().unwrap();

    let mut ret_supposed: [u8; 98_311] = [0; 98_311];
    ret_supposed[0] = 0x52;
    ret_supposed[1] = 0x80;
    ret_supposed[2] = 0x00;
    for i in 0..32_768 {
        ret_supposed[i * 3 + 3] = 0xe8;
        ret_supposed[i * 3 + 4] = 0xbf;
        ret_supposed[i * 3 + 5] = 0x99;
    }
    ret_supposed[98_307] = 0x1;
    ret_supposed[98_308] = 0xe8;
    ret_supposed[98_309] = 0xbf;
    ret_supposed[98_310] = 0x99;
    assert_eq!(ret_supposed, buf.get().deref());
}

#[test]
fn test_string_6_1() {
    let mut v_string: String = "".to_string();
    for _ in 0..33_792 {
        v_string.push('这');
    }
    let v: &str = v_string.as_str();

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_str(v).unwrap();
    buf.flush().unwrap();

    let mut ret_supposed: [u8; 101_382] = [0; 101_382];
    ret_supposed[0] = 0x52;
    ret_supposed[1] = 0x80;
    ret_supposed[2] = 0x00;
    for i in 0..32_768 {
        ret_supposed[i * 3 + 3] = 0xe8;
        ret_supposed[i * 3 + 4] = 0xbf;
        ret_supposed[i * 3 + 5] = 0x99;
    }
    ret_supposed[98_307] = 0x53;
    ret_supposed[98_308] = 0x4;
    ret_supposed[98_309] = 0x0;
    for i in 0..1024 {
        ret_supposed[i * 3 + 98_310] = 0xe8;
        ret_supposed[i * 3 + 98_311] = 0xbf;
        ret_supposed[i * 3 + 98_312] = 0x99;
    }
    assert_eq!(ret_supposed, buf.get().deref());
}

#[test]
fn test_binary_0_1() {
    const V: [u8; 0] = [];

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_bytes(&V).unwrap();
    buf.flush().unwrap();

    assert_eq!([0x20], buf.get().deref());
}

#[test]
fn test_binary_1_1() {
    const V: [u8; 1] = [1u8; 1];

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_bytes(&V).unwrap();
    buf.flush().unwrap();

    assert_eq!([0x21, 0x1], buf.get().deref());
}

#[test]
fn test_binary_1_2() {
    const V: [u8; 15] = [1u8; 15];

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_bytes(&V).unwrap();
    buf.flush().unwrap();

    let mut ret_supposed: [u8; 16] = [0; 16];
    ret_supposed[0] = 0x2f;
    for i in 1..16 {
        ret_supposed[i] = 0x1;
    }
    assert_eq!(ret_supposed, buf.get().deref());
}

#[test]
fn test_binary_2_1() {
    const V: [u8; 16] = [1u8; 16];

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_bytes(&V).unwrap();
    buf.flush().unwrap();

    let mut ret_supposed: [u8; 18] = [0; 18];
    ret_supposed[0] = 0x34;
    ret_supposed[1] = 0x10;
    for i in 2..18 {
        ret_supposed[i] = 0x1;
    }
    assert_eq!(ret_supposed, buf.get().deref());
}

#[test]
fn test_binary_2_2() {
    const V: [u8; 1_023] = [1u8; 1_023];

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_bytes(&V).unwrap();
    buf.flush().unwrap();

    let mut ret_supposed: [u8; 1_025] = [0; 1_025];
    ret_supposed[0] = 0x37;
    ret_supposed[1] = 0xff;
    for i in 2..1_025 {
        ret_supposed[i] = 0x1;
    }
    assert_eq!(ret_supposed, buf.get().deref());
}

#[test]
fn test_binary_3_1() {
    const V: [u8; 1_024] = [1u8; 1_024];

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_bytes(&V).unwrap();
    buf.flush().unwrap();

    let mut ret_supposed: [u8; 1_027] = [0; 1_027];
    ret_supposed[0] = 0x42;
    ret_supposed[1] = 0x4;
    ret_supposed[2] = 0x0;
    for i in 3..1_027 {
        ret_supposed[i] = 0x1;
    }
    assert_eq!(ret_supposed, buf.get().deref());
}

#[test]
fn test_binary_3_2() {
    const V: [u8; 2_048] = [1u8; 2_048];

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_bytes(&V).unwrap();
    buf.flush().unwrap();

    let mut ret_supposed: [u8; 2_051] = [0; 2_051];
    ret_supposed[0] = 0x42;
    ret_supposed[1] = 0x8;
    ret_supposed[2] = 0x0;
    for i in 3..2_051 {
        ret_supposed[i] = 0x1;
    }
    assert_eq!(ret_supposed, buf.get().deref());
}

#[test]
fn test_binary_3_3() {
    const V: [u8; 8_189] = [1u8; 8_189];

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_bytes(&V).unwrap();
    buf.flush().unwrap();

    let mut ret_supposed: [u8; 8_192] = [0; 8_192];
    ret_supposed[0] = 0x42;
    ret_supposed[1] = 0x1f;
    ret_supposed[2] = 0xfd;
    for i in 3..8_192 {
        ret_supposed[i] = 0x1;
    }
    assert_eq!(ret_supposed, buf.get().deref());
}

#[test]
fn test_binary_4_1() {
    const V: [u8; 8_190] = [1u8; 8_190];

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_bytes(&V).unwrap();
    buf.flush().unwrap();

    let mut ret_supposed: [u8; 8_194] = [0; 8_194];
    ret_supposed[0] = 0x41;
    ret_supposed[1] = 0x1f;
    ret_supposed[2] = 0xfd;
    for i in 3..8_192 {
        ret_supposed[i] = 0x1;
    }
    ret_supposed[8_192] = 0x21;
    ret_supposed[8_193] = 0x1;
    assert_eq!(ret_supposed, buf.get().deref());
}

#[test]
fn test_binary_4_2() {
    const V: [u8; 8_192] = [1u8; 8_192];

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_bytes(&V).unwrap();
    buf.flush().unwrap();

    let mut ret_supposed: [u8; 8_196] = [0; 8_196];
    ret_supposed[0] = 0x41;
    ret_supposed[1] = 0x1f;
    ret_supposed[2] = 0xfd;
    for i in 3..8_192 {
        ret_supposed[i] = 0x1;
    }
    ret_supposed[8_192] = 0x23;
    for i in 8_193..8_196 {
        ret_supposed[i] = 0x1;
    }
    assert_eq!(ret_supposed, buf.get().deref());
}

#[test]
fn test_binary_5_1() {
    const V: [u8; 8_205] = [1u8; 8_205];

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_bytes(&V).unwrap();
    buf.flush().unwrap();

    let mut ret_supposed: [u8; 8_210] = [0; 8_210];
    ret_supposed[0] = 0x41;
    ret_supposed[1] = 0x1f;
    ret_supposed[2] = 0xfd;
    for i in 3..8_192 {
        ret_supposed[i] = 0x1;
    }
    ret_supposed[8_192] = 0x34;
    ret_supposed[8_193] = 0x10;
    for i in 8_194..8_210 {
        ret_supposed[i] = 0x1;
    }
    assert_eq!(ret_supposed, buf.get().deref());
}

#[test]
fn test_binary_5_2() {
    const V: [u8; 9_212] = [1u8; 9_212];

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_bytes(&V).unwrap();
    buf.flush().unwrap();

    let mut ret_supposed: [u8; 9_217] = [0; 9_217];
    ret_supposed[0] = 0x41;
    ret_supposed[1] = 0x1f;
    ret_supposed[2] = 0xfd;
    for i in 3..8_192 {
        ret_supposed[i] = 0x1;
    }
    ret_supposed[8_192] = 0x37;
    ret_supposed[8_193] = 0xff;
    for i in 8_194..9_217 {
        ret_supposed[i] = 0x1;
    }
    assert_eq!(ret_supposed, buf.get().deref());
}

#[test]
fn test_binary_6_1() {
    const V: [u8; 9_213] = [1u8; 9_213];

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_bytes(&V).unwrap();
    buf.flush().unwrap();

    let mut ret_supposed: [u8; 9_219] = [0; 9_219];
    ret_supposed[0] = 0x41;
    ret_supposed[1] = 0x1f;
    ret_supposed[2] = 0xfd;
    for i in 3..8_192 {
        ret_supposed[i] = 0x1;
    }
    ret_supposed[8_192] = 0x42;
    ret_supposed[8_193] = 0x4;
    ret_supposed[8_194] = 0x0;
    for i in 8_195..9_219 {
        ret_supposed[i] = 0x1;
    }
    assert_eq!(ret_supposed, buf.get().deref());
}

#[test]
fn test_binary_7_1() {
    const V: [u8; 16_378] = [1u8; 16_378];

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_bytes(&V).unwrap();
    buf.flush().unwrap();

    let mut ret_supposed: [u8; 16_384] = [0; 16_384];
    ret_supposed[0] = 0x41;
    ret_supposed[1] = 0x1f;
    ret_supposed[2] = 0xfd;
    for i in 3..8_192 {
        ret_supposed[i] = 0x1;
    }
    ret_supposed[8_192] = 0x42;
    ret_supposed[8_193] = 0x1f;
    ret_supposed[8_194] = 0xfd;
    for i in 8_195..16_384 {
        ret_supposed[i] = 0x1;
    }
    assert_eq!(ret_supposed, buf.get().deref());
}

#[test]
fn test_binary_8_1() {
    const V: [u8; 16_379] = [1u8; 16_379];

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    ser.serialize_bytes(&V).unwrap();
    buf.flush().unwrap();

    let mut ret_supposed: [u8; 16_386] = [0; 16_386];
    ret_supposed[0] = 0x41;
    ret_supposed[1] = 0x1f;
    ret_supposed[2] = 0xfd;
    for i in 3..8_192 {
        ret_supposed[i] = 0x1;
    }
    ret_supposed[8_192] = 0x41;
    ret_supposed[8_193] = 0x1f;
    ret_supposed[8_194] = 0xfd;
    for i in 8_195..16_384 {
        ret_supposed[i] = 0x1;
    }
    ret_supposed[16_384] = 0x21;
    ret_supposed[16_385] = 0x1;
    assert_eq!(ret_supposed, buf.get().deref());
}

#[test]
fn test_typed_list_int_0_1() {
    let v: Vec<u8> = vec![];

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    let mut seq = ser.serialize_seq(Some(v.len())).unwrap();
    for element in v {
        seq.serialize_element(&element).unwrap();
    }
    seq.end().unwrap();
    buf.flush().unwrap();

    assert_eq!([0x78], buf.get().deref());
}


#[test]
fn test_typed_list_int_1_1() {
    let v: Vec<u8> = vec![1u8];

    let mut buf = BytesBufWriter::new();
    let mut ser = Serializer::new(&mut buf);
    let mut seq = ser.serialize_seq(Some(v.len())).unwrap();
    for element in v {
        seq.serialize_element(&element).unwrap();
    }
    seq.end().unwrap();
    buf.flush().unwrap();

    assert_eq!([0x71,0x4,0x5b,0x69,0x6e,0x74,0x91], buf.get().deref());
}
