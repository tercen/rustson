extern crate serde_derive;
extern crate bytes;
extern crate serde;
extern crate serde_json;

pub mod deser;
pub mod ser;

use std::io::Cursor;
use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use std::error;
use std::fmt;

use deser::Deserializer;
use ser::Serializer;

#[derive(Debug, Clone, PartialEq)]
pub struct TsonError {
    description: String
}

impl TsonError {
    pub fn new<T>(description: T) -> TsonError where T: Into<String>{
        TsonError { description: description.into() }
    }

    pub fn other<T>(e: T ) -> TsonError where T: error::Error {
        TsonError { description: e.to_string().to_owned() }
    }
}

impl fmt::Display for TsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}", &self.description)
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

pub type Result<T> = std::result::Result<T, TsonError>;


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

    LSTSTR(Vec<String>),
}



pub fn encode_json(value: &Value) -> Result<String> {
    serde_json::to_string(&value).map_err(|e| TsonError::new(format!("encode_json  : failed with {}", e)))
}

pub fn decode_json(v: &[u8]) -> Result<Value> {
    serde_json::from_slice(v).map_err(|e|TsonError::new(format!("decode_json : failed with {}", e)) )
}

pub fn encode(value: &Value) -> Result<Vec<u8>> {
    let ser = Serializer::new();
    ser.encode(value)
}

pub fn decode(mut cur: Cursor<&[u8]>) -> Result<Value> {
    let deser = Deserializer::new();
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

        let ser = Serializer::new();
        println!("p encoded_size {:#?}", ser.encoded_size(&p));
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
