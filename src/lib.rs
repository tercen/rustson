extern crate serde_derive;
extern crate bytes;
extern crate serde;
extern crate serde_json;
extern crate byteorder;

pub mod deser;
pub mod ser;
pub mod spec;
pub mod gdeser;

use std::io::{Cursor, Error};
use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use std::error;
use std::fmt;

use deser::{Deserializer, Reader};
use ser::Serializer;
use std::convert::TryInto;

use spec::*;

pub static VERSION: &'static str = "1.1.0";

pub type TsonResult<T> = std::result::Result<T, TsonError>;

#[derive(Debug, Clone, PartialEq)]
pub struct TsonError {
    description: String
}

impl TsonError {
    pub fn new<T>(description: T) -> TsonError where T: Into<String> {
        TsonError { description: description.into() }
    }

    pub fn other<T>(e: T) -> TsonError where T: error::Error {
        TsonError { description: e.to_string().to_owned() }
    }
}

impl From<std::io::Error> for TsonError {
    fn from(value: Error) -> Self {
        TsonError::new(value.to_string())
    }
}

impl fmt::Display for TsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.description)
    }
}

// This is important for other errors to wrap this one.
impl error::Error for TsonError {
    fn description(&self) -> &str {
        &self.description
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> { None }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct StrVec {
    pub bytes: Vec<u8>,
}

impl StrVec {
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        StrVec { bytes }
    }
    pub fn build_starts(&self) -> TsonResult<Vec<usize>> {
        let mut reader = Cursor::new(&self.bytes);
        let mut len_in_bytes = self.bytes.len();
        let mut vec = Vec::with_capacity(len_in_bytes);

        let mut start = 0usize;
        vec.push(start);

        while len_in_bytes > 0 {
            let len = read_string_len(&mut reader)?;
            start += len + 1;
            vec.push(start);
            len_in_bytes -= len + 1;
        }
        vec.shrink_to_fit();
        Ok(vec)
    }

    pub fn try_to_vec(&self) -> TsonResult<Vec<String>> {
        let mut reader = Cursor::new(&self.bytes);
        let mut len_in_bytes = self.bytes.len();
        let mut vec = Vec::new();
        while len_in_bytes > 0 {
            let v = read_string(&mut reader)?;
            len_in_bytes -= v.as_bytes().len() + 1;
            vec.push(v);
        }
        Ok(vec)
    }
}

fn read_string(reader: &mut dyn Reader) -> TsonResult<String> {
    let mut done = false;
    let mut vec = Vec::new();
    while !done {
        let byte = reader.read_u8()?;
        if byte == 0 {
            done = true;
        } else {
            vec.push(byte);
        }
    }

    if let Ok(value) = String::from_utf8(vec) {
        Ok(value)
    } else {
        Err(TsonError::new("utf8 : bad string"))
    }
}

fn read_string_len(reader: &mut dyn Reader) -> TsonResult<usize> {
    let mut done = false;
    let mut len = 0;
    while !done {
        let byte = reader.read_u8()?;
        if byte == 0 {
            done = true;
        } else {
            len += 1;
        }
    }

    Ok(len)
}


impl TryInto<Vec<String>> for StrVec {
    type Error = TsonError;

    fn try_into(self) -> TsonResult<Vec<String>> {
        let mut reader = Cursor::new(&self.bytes);
        let mut len_in_bytes = self.bytes.len();
        let mut vec = Vec::new();
        while len_in_bytes > 0 {
            let v = read_string(&mut reader)?;
            len_in_bytes -= v.as_bytes().len() + 1;
            vec.push(v);
        }
        Ok(vec)
    }
}


impl Into<StrVec> for Vec<String> {
    fn into(self) -> StrVec {
        let len_in_bytes = self.iter().map(|e| e.as_bytes().len() + 1).sum();
        let mut bytes = Vec::with_capacity(len_in_bytes);
        self.iter().for_each(|e| {
            bytes.extend(e.as_bytes());
            bytes.push(0);
        });
        StrVec { bytes }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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

    LSTSTR(StrVec),
}

impl Value {
    pub fn to_bytes(&self) -> TsonResult<Vec<u8>> {
        encode(self)
    }

    pub fn to_str(&self) -> TsonResult<&str> {
        match *self {
            Value::STR(ref v) => {
                Ok(v.as_str())
            }
            _ => Err(TsonError::new("str expected"))
        }
    }

    pub fn to_map(&self) -> TsonResult<&HashMap<String, Value>> {
        match *self {
            Value::MAP(ref v) => {
                Ok(v)
            }
            _ => Err(TsonError::new("str expected"))
        }
    }

    pub fn to_list(&self) -> TsonResult<&Vec<Value>> {
        match *self {
            Value::LST(ref v) => {
                Ok(v)
            }
            _ => Err(TsonError::new("lst expected"))
        }
    }
}

pub fn encode_json(value: &Value) -> TsonResult<String> {
    serde_json::to_string(&value).map_err(|e| TsonError::new(format!("encode_json  : failed with {}", e)))
}

pub fn decode_json(v: &[u8]) -> TsonResult<Value> {
    serde_json::from_slice(v).map_err(|e| TsonError::new(format!("decode_json : failed with {}", e)))
}

pub fn encode(value: &Value) -> TsonResult<Vec<u8>> {
    let ser = Serializer::new();
    ser.encode(value)
}

pub fn decode(mut cur: Cursor<&[u8]>) -> TsonResult<Value> {
    let deser = Deserializer::new();
    deser.read(&mut cur)
}

pub fn decode_bytes(bytes: &[u8]) -> TsonResult<Value> {
    let deser = Deserializer::new();
    let mut cur = Cursor::new(&bytes);
    deser.read(&mut cur)
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
        vec.push(Value::STR("".to_owned()));
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
        vec.push(Value::LSTSTR(vec!["42".to_owned()].into()));


        let object = Value::LST(vec);


        let j = encode_json(&object).unwrap();
        println!("{}", j);

        encode_decode(&object);

        let data = r#"[null,true,42,42.0,"42.0",[42],[42],[42],[42],[42],[42],[42],[42],[42.0],[42.0],["42"]]"#;

        let p = decode_json(data.to_string().as_bytes()).unwrap();

        // println!("{:#?}", p);

        let ser = Serializer::new();
        // println!("p encoded_size {:#?}", ser.encoded_size(&p));
    }

    #[test]
    fn empty_string() {
        let mut vec = Vec::new();
        vec.push(Value::STR("".to_owned()));
        let object = Value::LST(vec);
        let bytes = encode(&object).unwrap();
        println!("{:#?}", bytes);
        encode_decode(&Value::STR("".to_owned()));
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
