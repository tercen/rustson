extern crate bytes;

use super::*;

use bytes::{BufMut};
use std::slice;
//use std::mem::size_of;

pub trait Writer {
    fn add_u8(&mut self, value: u8) -> Result<()>;
    fn add_i8(&mut self, value: i8) -> Result<()>;
    fn add_u32(&mut self, value: u32) -> Result<()>;
    fn add_i32(&mut self, value: i32) -> Result<()>;
    fn add_f64(&mut self, value: f64) -> Result<()>;
    fn add_u16(&mut self, value: u16) -> Result<()>;
    fn add_i16(&mut self, value: i16) -> Result<()>;
    fn add_u64(&mut self, value: u64) -> Result<()>;
    fn add_i64(&mut self, value: i64) -> Result<()>;
    fn add_f32(&mut self, value: f32) -> Result<()>;
    fn put_slice(&mut self, src: &[u8]) -> Result<()>;
}

impl<T> Writer for T where T: BufMut {
    fn add_u8(&mut self, value: u8) -> Result<()> {
        self.put_u8(value);
        Ok(())
    }
    fn add_i8(&mut self, value: i8) -> Result<()> {
        self.put_i8(value);
        Ok(())
    }
    fn add_u32(&mut self, value: u32) -> Result<()> {
        self.put_u32_le(value);
        Ok(())
    }
    fn add_i32(&mut self, value: i32) -> Result<()> {
        self.put_i32_le(value);
        Ok(())
    }
    fn add_f64(&mut self, value: f64) -> Result<()> {
        self.put_f64_le(value);
        Ok(())
    }
    fn add_u16(&mut self, value: u16) -> Result<()> {
        self.put_u16_le(value);
        Ok(())
    }
    fn add_i16(&mut self, value: i16) -> Result<()> {
        self.put_i16_le(value);
        Ok(())
    }
    fn add_u64(&mut self, value: u64) -> Result<()> {
        self.put_u64_le(value);
        Ok(())
    }
    fn add_i64(&mut self, value: i64) -> Result<()> {
        self.put_i64_le(value);
        Ok(())
    }
    fn add_f32(&mut self, value: f32) -> Result<()> {
        self.put_f32_le(value);
        Ok(())
    }

    fn put_slice(&mut self, value: &[u8]) -> Result<()> {

        self.put_slice(value);
        Ok(())
    }
}

pub struct CountWriter {
    pub size: usize,
}

impl CountWriter {
    pub fn new() -> CountWriter {
        CountWriter { size: 0 }
    }
}

impl Writer for CountWriter {
    fn add_u8(&mut self, _value: u8) -> Result<()> {
        self.size += 1;
        Ok(())
    }
    fn add_i8(&mut self, _value: i8) -> Result<()> {
        self.size += 1;
        Ok(())
    }
    fn add_u32(&mut self, _value: u32) -> Result<()> {
        self.size += 4;
        Ok(())
    }
    fn add_i32(&mut self, _value: i32) -> Result<()> {
        self.size += 4;
        Ok(())
    }
    fn add_f64(&mut self, _value: f64) -> Result<()> {
        self.size += 8;
        Ok(())
    }
    fn add_u16(&mut self, _value: u16) -> Result<()> {
        self.size += 2;
        Ok(())
    }
    fn add_i16(&mut self, _value: i16) -> Result<()> {
        self.size += 2;
        Ok(())
    }
    fn add_u64(&mut self, _value: u64) -> Result<()> {
        self.size += 8;
        Ok(())
    }
    fn add_i64(&mut self, _value: i64) -> Result<()> {
        self.size += 8;
        Ok(())
    }
    fn add_f32(&mut self, _value: f32) -> Result<()> {
        self.size += 4;
        Ok(())
    }

    fn put_slice(&mut self, src: &[u8]) -> Result<()> {
        self.size += src.len();
        Ok(())
    }
}

pub struct Serializer {}

impl Serializer {
    pub fn new() -> Serializer { Serializer {} }

    pub fn encoded_size(&self, value: &Value) -> Result<usize> {
        let mut buf = CountWriter::new();
        self.add_string(&mut buf, VERSION)?;

        match self.add_object(value, &mut buf) {
            Ok(_) => Ok(buf.size),
            Err(e) => Err(e),
        }
    }

    pub fn encode(&self, value: &Value) -> Result<Vec<u8>> {
        let size = self.encoded_size(value)?;

        let mut buf = Vec::with_capacity(size);
        self.add_string(&mut buf, VERSION)?;

        match self.add_object(value, &mut buf) {
            Ok(_) => Ok(buf),
            Err(e) => Err(e),
        }
    }

    pub fn write(&self, value: &Value, writer: &mut dyn Writer) -> Result<()> {
        self.add_string(writer, VERSION)?;
        self.add_object(value, writer)
    }

    fn add_object(&self, value: &Value, buf: &mut dyn Writer) -> Result<()> {
        match *value {
            Value::NULL => {
                buf.add_u8(NULL_TYPE)?;
            }
            Value::STR(ref v) => {
                self.add_string(buf, v)?;
            }
            Value::I32(v) => {
                buf.add_u8(INTEGER_TYPE)?;
                buf.add_i32(v)?;
            }
            Value::F64(v) => {
                buf.add_u8(DOUBLE_TYPE)?;
                buf.add_f64(v)?;
            }
            Value::BOOL(v) => {
                buf.add_u8(BOOL_TYPE)?;
                if v {
                    buf.add_u8(1)?;
                } else {
                    buf.add_u8(0)?;
                }
            }
            Value::LST(ref v) => {
                buf.add_u8(LIST_TYPE)?;
                self.add_len(buf, v.len())?;
                for object in v.iter() {
                    self.add_object(object, buf)?;
                }
            }
            Value::MAP(ref v) => {
                buf.add_u8(MAP_TYPE)?;
                self.add_len(buf, v.len())?;
                for (k, v) in v.iter() {
                    self.add_string(buf, k)?;
                    self.add_object(v, buf)?;
                }
            }
            Value::LSTU8(ref v) => {
                buf.add_u8(LIST_UINT8_TYPE)?;
                self.add_len(buf, v.len())?;
                buf.put_slice(&v)?;
                // for i in v.iter() {
                //     buf.add_u8(*i)?;
                // }
            }
            Value::LSTI8(ref v) => {
                buf.add_u8(LIST_INT8_TYPE)?;
                self.add_len(buf, v.len())?;
                let slice = unsafe { slice::from_raw_parts(v.as_ptr() as *mut u8, v.len() * std::mem::size_of::<i8>()) };
                buf.put_slice(slice)?;
                // for i in v.iter() {
                //     buf.add_i8(*i)?;
                // }
            }
            Value::LSTU16(ref v) => {
                buf.add_u8(LIST_UINT16_TYPE)?;
                self.add_len(buf, v.len())?;
                let slice = unsafe { slice::from_raw_parts(v.as_ptr() as *mut u8, v.len() * std::mem::size_of::<u16>()) };
                buf.put_slice(slice)?;
                // for i in v.iter() {
                //     buf.add_u16(*i)?;
                // }
            }
            Value::LSTI16(ref v) => {
                buf.add_u8(LIST_INT16_TYPE)?;
                self.add_len(buf, v.len())?;
                let slice = unsafe { slice::from_raw_parts(v.as_ptr() as *mut u8, v.len() * std::mem::size_of::<i16>()) };
                buf.put_slice(slice)?;
                // for i in v.iter() {
                //     buf.add_i16(*i)?;
                // }
            }
            Value::LSTU32(ref v) => {
                buf.add_u8(LIST_UINT32_TYPE)?;
                self.add_len(buf, v.len())?;
                let slice = unsafe { slice::from_raw_parts(v.as_ptr() as *mut u8, v.len() * std::mem::size_of::<u32>()) };
                buf.put_slice(slice)?;
                // for i in v.iter() {
                //     buf.add_u32(*i)?;
                // }
            }
            Value::LSTI32(ref v) => {
                buf.add_u8(LIST_INT32_TYPE)?;
                self.add_len(buf, v.len())?;
                let slice = unsafe { slice::from_raw_parts(v.as_ptr() as *mut u8, v.len() * std::mem::size_of::<i32>()) };
                buf.put_slice(slice)?;
                // for i in v.iter() {
                //     buf.add_i32(*i)?;
                // }
            }
            Value::LSTU64(ref v) => {
                buf.add_u8(LIST_UINT64_TYPE)?;
                self.add_len(buf, v.len())?;
                for i in v.iter() {
                    buf.add_u64(*i)?;
                }
            }
            Value::LSTI64(ref v) => {
                buf.add_u8(LIST_INT64_TYPE)?;
                self.add_len(buf, v.len())?;
                let slice = unsafe { slice::from_raw_parts(v.as_ptr() as *mut u8, v.len() * std::mem::size_of::<i64>()) };
                buf.put_slice(slice)?;
                // for i in v.iter() {
                //     buf.add_i64(*i)?;
                // }
            }
            Value::LSTF32(ref v) => {
                buf.add_u8(LIST_FLOAT32_TYPE)?;
                self.add_len(buf, v.len())?;
                let slice = unsafe { slice::from_raw_parts(v.as_ptr() as *mut u8, v.len() * std::mem::size_of::<f32>()) };
                buf.put_slice(slice)?;
                // for i in v.iter() {
                //     buf.add_f32(*i)?;
                // }
            }
            Value::LSTF64(ref v) => {
                buf.add_u8(LIST_FLOAT64_TYPE)?;
                self.add_len(buf, v.len())?;
                let slice = unsafe { slice::from_raw_parts(v.as_ptr() as *mut u8, v.len() * std::mem::size_of::<f64>()) };
                buf.put_slice(slice)?;
                // for i in v.iter() {
                //     buf.add_f64(*i)?;
                // }
            }
            Value::LSTSTR(ref v) => {
                buf.add_u8(LIST_STRING_TYPE)?;
                let len_in_bytes = v.bytes.len();
                self.add_len(buf, len_in_bytes)?;

                for i in v.bytes.iter() {
                    buf.add_u8(*i)?;
                }
            }
        }

        Ok(())
    }


    fn add_len(&self, buf: &mut dyn Writer, len: usize) -> Result<()> {
        if len > MAX_LIST_LENGTH {
            return Err(TsonError::new("list too large"));
        }
        buf.add_u32(len as u32)
    }

    fn add_string(&self, buf: &mut dyn Writer, value: &str) -> Result<()> {
        buf.add_u8(STRING_TYPE)?;
        self.add_cstring(buf, value)
    }

    fn add_cstring(&self, buf: &mut dyn Writer, value: &str) -> Result<()> {
        for byte in value.as_bytes().iter() {
            buf.add_u8(*byte)?;
        }
        buf.add_u8(0)
    }
}
