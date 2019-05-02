extern crate bytes;

use super::*;

use bytes::{BufMut};

pub trait Writer {
    fn add_u8(&mut self, value: u8);
    fn add_i8(&mut self, value: i8);
    fn add_u32(&mut self, value: u32);
    fn add_i32(&mut self, value: i32);
    fn add_f64(&mut self, value: f64);
    fn add_u16(&mut self, value: u16);
    fn add_i16(&mut self, value: i16);
    fn add_u64(&mut self, value: u64);
    fn add_i64(&mut self, value: i64);
    fn add_f32(&mut self, value: f32);

}

impl<T> Writer for T where T: BufMut {
    fn add_u8(&mut self, value: u8) {
        self.put_u8(value);
    }
    fn add_i8(&mut self, value: i8) {
        self.put_i8(value);
    }
    fn add_u32(&mut self, value: u32) {
        self.put_u32_le(value);
    }
    fn add_i32(&mut self, value: i32) {
        self.put_i32_le(value);
    }
    fn add_f64(&mut self, value: f64) {
        self.put_f64_le(value);
    }
    fn add_u16(&mut self, value: u16) {
        self.put_u16_le(value);
    }
    fn add_i16(&mut self, value: i16) {
        self.put_i16_le(value);
    }
    fn add_u64(&mut self, value: u64) {
        self.put_u64_le(value);
    }
    fn add_i64(&mut self, value: i64) {
        self.put_i64_le(value);
    }
    fn add_f32(&mut self, value: f32) {
        self.put_f32_le(value);
    }


}

pub struct Serializer {}

impl Serializer {
    pub fn new() -> Serializer {Serializer{}}

    pub fn encode(&self, value: &Value) -> Result<Vec<u8>, String> {
        let mut buf = Vec::new();
        self.add_string(&mut buf, VERSION);

        match self.add_object(value, &mut buf) {
            Ok(_) => Ok(buf),
            Err(e) => Err(e),
        }
    }

    pub fn write(&self, value: &Value, writer: &mut Writer) -> Result<(), String> {
        self.add_string(writer, VERSION);
        match self.add_object(value, &mut buf) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn add_object(&self, value: &Value, buf: &mut Writer) -> Result<(), String> {
        match *value {
            Value::NULL => {
                buf.add_u8(NULL_TYPE);
            }
            Value::STR(ref v) => {
                self.add_string(buf, v);
            }
            Value::I32(v) => {
                buf.add_u8(INTEGER_TYPE);
                buf.add_i32(v);
            }
            Value::F64(v) => {
                buf.add_u8(DOUBLE_TYPE);
                buf.add_f64(v);
            }
            Value::BOOL(v) => {
                buf.add_u8(BOOL_TYPE);
                if v {
                    buf.add_u8(1);
                } else {
                    buf.add_u8(0);
                }
            }
            Value::LST(ref v) => {
                buf.add_u8(LIST_TYPE);
                self.add_len(buf, v.len())?;
                for object in v.iter() {
                    self.add_object(object, buf)?;
                }
            }
            Value::MAP(ref v) => {
                buf.add_u8(MAP_TYPE);
                self.add_len(buf, v.len())?;
                for (k, v) in v.iter() {
                    self.add_string(buf, k);
                    self.add_object(v, buf)?;
                }
            }
            Value::LSTU8(ref v) => {
                buf.add_u8(LIST_UINT8_TYPE);
                self.add_len(buf, v.len())?;
                for i in v.iter() {
                    buf.add_u8(*i);
                }
            }
            Value::LSTI8(ref v) => {
                buf.add_u8(LIST_INT8_TYPE);
                self.add_len(buf, v.len())?;
                for i in v.iter() {
                    buf.add_i8(*i);
                }
            }
            Value::LSTU16(ref v) => {
                buf.add_u8(LIST_UINT16_TYPE);
                self.add_len(buf, v.len())?;
                for i in v.iter() {
                    buf.add_u16(*i);
                }
            }
            Value::LSTI16(ref v) => {
                buf.add_u8(LIST_INT16_TYPE);
                self.add_len(buf, v.len())?;
                for i in v.iter() {
                    buf.add_i16(*i);
                }
            }
            Value::LSTU32(ref v) => {
                buf.add_u8(LIST_UINT32_TYPE);
                self.add_len(buf, v.len())?;
                for i in v.iter() {
                    buf.add_u32(*i);
                }
            }
            Value::LSTI32(ref v) => {
                buf.add_u8(LIST_INT32_TYPE);
                self.add_len(buf, v.len())?;
                for i in v.iter() {
                    buf.add_i32(*i);
                }
            }
            Value::LSTU64(ref v) => {
                buf.add_u8(LIST_UINT64_TYPE);
                self.add_len(buf, v.len())?;
                for i in v.iter() {
                    buf.add_u64(*i);
                }
            }
            Value::LSTI64(ref v) => {
                buf.add_u8(LIST_INT64_TYPE);
                self.add_len(buf, v.len())?;
                for i in v.iter() {
                    buf.add_i64(*i);
                }
            }
            Value::LSTF32(ref v) => {
                buf.add_u8(LIST_FLOAT32_TYPE);
                self.add_len(buf, v.len())?;
                for i in v.iter() {
                    buf.add_f32(*i);
                }
            }
            Value::LSTF64(ref v) => {
                buf.add_u8(LIST_FLOAT64_TYPE);
                self.add_len(buf, v.len())?;
                for i in v.iter() {
                    buf.add_f64(*i);
                }
            }
            Value::LSTSTR(ref v) => {
                buf.add_u8(LIST_STRING_TYPE);
                let mut len_in_bytes = 0;
                for i in v.iter() {
                    len_in_bytes += i.as_bytes().len() + 1;
                }
                self.add_len(buf, len_in_bytes)?;

                for i in v.iter() {
                    self.add_cstring(buf, i);
                }
            }
        }

        Ok(())
    }


    fn add_len(&self, buf: &mut Writer, len: usize) -> Result<(), String> {
        if len > (std::u32::MAX as usize) {
            return Err("list too large".to_owned());
        }
        buf.add_u32(len as u32);
        Ok(())
    }

    fn add_string(&self, buf: &mut Writer, value: &str) {
        buf.add_u8(STRING_TYPE);
        self.add_cstring(buf, value);
    }

    fn add_cstring(&self, buf: &mut Writer, value: &str) {
        for byte in value.as_bytes().iter() {
            buf.add_u8(*byte);
        }
        buf.add_u8(0);
    }
}
