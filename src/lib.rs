extern crate serde_derive;
extern crate bytes;

extern crate serde;

extern crate serde_json;

use std::io::Cursor;
use std::collections::HashMap;

use bytes::{Buf};

use serde::{Serialize, Deserialize};

mod ser;

use ser::Serializer;

pub static VERSION: &'static str = "1.1.0";

pub const NULL_TYPE: u8 = 0;
pub const STRING_TYPE: u8 = 1;
pub const INTEGER_TYPE: u8 = 2;
pub const DOUBLE_TYPE: u8 = 3;
pub const BOOL_TYPE: u8 = 4;

pub const LIST_TYPE: u8 = 10;
pub const MAP_TYPE: u8 = 11;

pub const LIST_UINT8_TYPE: u8 = 100;
pub const LIST_UINT16_TYPE: u8 = 101;
pub const LIST_UINT32_TYPE: u8 = 102;

pub const LIST_INT8_TYPE: u8 = 103;
pub const LIST_INT16_TYPE: u8 = 104;
pub const LIST_INT32_TYPE: u8 = 105;
pub const LIST_INT64_TYPE: u8 = 106;
pub const LIST_UINT64_TYPE: u8 = 107;

pub const LIST_FLOAT32_TYPE: u8 = 110;
pub const LIST_FLOAT64_TYPE: u8 = 111;

pub const LIST_STRING_TYPE: u8 = 112;

pub const MAX_LIST_LENGTH: usize = std::u32::MAX as usize;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum Value {
    NULL,
    STR(String),
    I32(i32),
    F64(f64),
    BOOL(bool),
    LST(Vec<Value>),
    MAP(HashMap<String, Value>),

    LSTU8(Vec<u8>),
    LSTI8(Vec<i8>),

    LSTU16(Vec<u16>),
    LSTI16(Vec<i16>),

    LSTU32(Vec<u32>),
    LSTI32(Vec<i32>),

    LSTU64(Vec<u64>),
    LSTI64(Vec<i64>),

    LSTF32(Vec<f32>),
    LSTF64(Vec<f64>),

    LSTSTR(Vec<String>),
}

pub fn encode_json(value: &Value) -> Result<String, String> {
    match serde_json::to_string(&value){
        Ok(j)=>Ok(j),
        Err(e)=>Err(format!("encode_json : failed with {}", e).to_string())
    }
}

pub fn decode_json(v: &[u8]) -> Result<Value, String> {
    match serde_json::from_slice(v){
        Ok(j)=>Ok(j),
        Err(e)=>Err(format!("decode_json : failed with {}", e).to_string())
    }
}

pub fn encode(value: &Value) -> Result<Vec<u8>, String> {
    let ser = Serializer::new();
    ser.encode(value)
}

pub fn decode(mut cur: Cursor<&[u8]>) -> Result<Value, String> {
    if cur.remaining() < 1 {
        return Err("wrong format".to_owned());
    }

    let itype = read_type(&mut cur)?;

    if itype != STRING_TYPE {
        return Err("wrong format".to_owned());
    }

    let version = read_string(&mut cur)?;

    if !version.eq(VERSION) {
        return Err("wrong version".to_owned());
    }

    read_object(&mut cur)
}

fn read_object(cur: &mut Cursor<&[u8]>) -> Result<Value, String> {
    let itype = read_type(cur)?;
    match itype {
        NULL_TYPE => Ok(Value::NULL),
        STRING_TYPE => Ok(Value::STR(read_string(cur)?)),
        INTEGER_TYPE => {
            if cur.remaining() < 4 {
                return Err("wrong format".to_owned());
            }
            Ok(Value::I32(cur.get_i32_le()))
        }
        DOUBLE_TYPE => {
            if cur.remaining() < 8 {
                return Err("wrong format".to_owned());
            }
            Ok(Value::F64(cur.get_f64_le()))
        }
        BOOL_TYPE => {
            if cur.remaining() < 1 {
                return Err("wrong format".to_owned());
            }
            Ok(Value::BOOL(cur.get_u8() > 0))
        }
        LIST_TYPE => {
            let len = read_len(cur)?;
            let mut vec = Vec::with_capacity(len);
            for _ in 0..len {
                vec.push(read_object(cur)?);
            }
            Ok(Value::LST(vec))
        }
        MAP_TYPE => {
            let len = read_len(cur)?;
            let mut map = HashMap::with_capacity(len);
            for _ in 0..len {
                if let Value::STR(k) = read_object(cur)? {
                    map.insert(k, read_object(cur)?);
                } else {
                    return Err("wrong format".to_owned());
                }
            }
            Ok(Value::MAP(map))
        }
        LIST_UINT8_TYPE => {
            let len = read_len(cur)?;
            if cur.remaining() < len {
                return Err("wrong format".to_owned());
            }
            let mut vec = Vec::with_capacity(len);
            for _ in 0..len {
                vec.push(cur.get_u8());
            }
            Ok(Value::LSTU8(vec))
        }
        LIST_INT8_TYPE => {
            let len = read_len(cur)?;
            if cur.remaining() < len {
                return Err("wrong format".to_owned());
            }
            let mut vec = Vec::with_capacity(len);
            for _ in 0..len {
                vec.push(cur.get_i8());
            }
            Ok(Value::LSTI8(vec))
        }
        LIST_UINT16_TYPE => {
            let len = read_len(cur)?;
            if cur.remaining() < (len * 2) {
                return Err("wrong format".to_owned());
            }
            let mut vec = Vec::with_capacity(len);
            for _ in 0..len {
                vec.push(cur.get_u16_le());
            }
            Ok(Value::LSTU16(vec))
        }
        LIST_INT16_TYPE => {
            let len = read_len(cur)?;
            if cur.remaining() < (len * 2) {
                return Err("wrong format".to_owned());
            }
            let mut vec = Vec::with_capacity(len);
            for _ in 0..len {
                vec.push(cur.get_i16_le());
            }
            Ok(Value::LSTI16(vec))
        }

        LIST_UINT32_TYPE => {
            let len = read_len(cur)?;
            if cur.remaining() < (len * 4) {
                return Err("wrong format".to_owned());
            }
            let mut vec = Vec::with_capacity(len);
            for _ in 0..len {
                vec.push(cur.get_u32_le());
            }
            Ok(Value::LSTU32(vec))
        }
        LIST_INT32_TYPE => {
            let len = read_len(cur)?;
            if cur.remaining() < (len * 4) {
                return Err("wrong format".to_owned());
            }
            let mut vec = Vec::with_capacity(len);
            for _ in 0..len {
                vec.push(cur.get_i32_le());
            }
            Ok(Value::LSTI32(vec))
        }
        LIST_INT64_TYPE => {
            let len = read_len(cur)?;
            if cur.remaining() < (len * 8) {
                return Err("wrong format".to_owned());
            }
            let mut vec = Vec::with_capacity(len);
            for _ in 0..len {
                vec.push(cur.get_i64_le());
            }
            Ok(Value::LSTI64(vec))
        }
        LIST_UINT64_TYPE => {
            let len = read_len(cur)?;
            if cur.remaining() < (len * 8) {
                return Err("wrong format".to_owned());
            }
            let mut vec = Vec::with_capacity(len);
            for _ in 0..len {
                vec.push(cur.get_u64_le());
            }
            Ok(Value::LSTU64(vec))
        }
        LIST_FLOAT32_TYPE => {
            let len = read_len(cur)?;
            if cur.remaining() < (len * 4) {
                return Err("wrong format".to_owned());
            }
            let mut vec = Vec::with_capacity(len);
            for _ in 0..len {
                vec.push(cur.get_f32_le());
            }
            Ok(Value::LSTF32(vec))
        }
        LIST_FLOAT64_TYPE => {
            let len = read_len(cur)?;
            if cur.remaining() < (len * 8) {
                return Err("wrong format".to_owned());
            }
            let mut vec = Vec::with_capacity(len);
            for _ in 0..len {
                vec.push(cur.get_f64_le());
            }
            Ok(Value::LSTF64(vec))
        }
        LIST_STRING_TYPE => {
            let mut len_in_bytes = read_len(cur)?;

            if cur.remaining() < len_in_bytes {
                return Err("wrong format".to_owned());
            }
            let mut vec = Vec::new();
            while len_in_bytes > 0 {
                let v = read_string(cur)?;
                len_in_bytes -= v.as_bytes().len() + 1;
                vec.push(v);
            }

            if len_in_bytes > 0 {
                return Err("wrong format".to_owned());
            }

            Ok(Value::LSTSTR(vec))
        }

        _ => Err("wrong format".to_owned()),
    }
}

fn read_type(cur: &mut Cursor<&[u8]>) -> Result<u8, String> {
    if cur.remaining() < 1 {
        return Err("wrong format".to_owned());
    }
    Ok(cur.get_u8())
}

fn read_len(cur: &mut Cursor<&[u8]>) -> Result<usize, String> {
    if cur.remaining() < 4 {
        return Err("wrong format".to_owned());
    }
    Ok(cur.get_u32_le() as usize)
}


fn read_string(cur: &mut Cursor<&[u8]>) -> Result<String, String> {
    let mut rem = cur.remaining();
    let mut vec = Vec::new();
    while rem > 0 {
        let byte = cur.get_u8();
        if byte == 0 {
            rem = 0;
        } else {
            vec.push(byte);
            rem -= 1;
        }
    }

    if let Ok(value) = String::from_utf8(vec) {
        return Ok(value);
    } else {
        return Err("bad string".to_owned());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
//    use std::fs::File;
//    use std::io::prelude::*;
//    use test::Bencher;

    fn encode_decode(object: &Value) {
        let bytes = encode(object).unwrap();
        let value = decode(Cursor::new(&bytes)).unwrap();
        assert_eq!(object, &value);
    }

    #[test]
    fn null() {
        encode_decode(&Value::NULL)
    }

    #[test]
    fn empty_lst() {
        let lst = Value::LST(Vec::new());
        encode_decode(&lst)
    }

    #[test]
    fn lst() {
        let mut vec = Vec::new();
        vec.push(Value::NULL);
        vec.push(Value::BOOL(true));
        vec.push(Value::I32(42));
        vec.push(Value::F64(42.0));
        vec.push(Value::STR("42.0".to_owned()));
        vec.push(Value::LSTU8(vec![42]));
        vec.push(Value::LSTI8(vec![42]));
        vec.push(Value::LSTU16(vec![42]));
        vec.push(Value::LSTI16(vec![42]));
        vec.push(Value::LSTU32(vec![42]));
        vec.push(Value::LSTI32(vec![42]));
        vec.push(Value::LSTU64(vec![42]));
        vec.push(Value::LSTI64(vec![42]));
        vec.push(Value::LSTF32(vec![42.0]));
        vec.push(Value::LSTF64(vec![42.0]));
        vec.push(Value::LSTSTR(vec!["42".to_owned()]));


        let object = Value::LST(vec);


        let j = encode_json(&object).unwrap();
        println!("{}", j);

        encode_decode(&object);

        let data = r#"[null,true,42,42.0,"42.0",[42],[42],[42],[42],[42],[42],[42],[42],[42.0],[42.0],["42"]]"#;

        let p = decode_json(data.to_string().as_bytes()).unwrap();

        println!("{:#?}", p);
    }

    #[test]
    fn map() {
        let mut map = HashMap::new();
        map.insert("i42".to_owned(), Value::I32(42));

        let mut inner_map = HashMap::new();
        inner_map.insert("u42".to_owned(), Value::LSTU8(vec![42]));

        map.insert("map".to_owned(), Value::MAP(inner_map));

        encode_decode(&Value::MAP(map))
    }

//    #[bench]
//    fn bench(b: &mut Bencher) {
//        let mut f = File::open("../dtson/bin/test_data.tson").expect("file not found");
//
//        let mut bytes = Vec::new();
//        f.read_to_end(&mut bytes);
//
//        b.iter(|| decode(Cursor::new(&bytes)).unwrap());
//    }
}
