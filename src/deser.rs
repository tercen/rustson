extern crate bytes;

use std::io::Read;
use super::*;

use bytes::{Buf, LittleEndian, ByteOrder};

pub trait Reader {
//    fn content_length(&self) -> Result<Option<usize>> {None}
    fn read_all(&mut self, buf: &mut Vec<u8>) -> Result<()>;
    fn read_u8(&mut self) -> Result<u8>;
    fn read_i8(&mut self) -> Result<i8>;
    fn read_u16(&mut self) -> Result<u16>;
    fn read_i16(&mut self) -> Result<i16>;
    fn read_u32(&mut self) -> Result<u32>;
    fn read_i32(&mut self) -> Result<i32>;
    fn read_u64(&mut self) -> Result<u64>;
    fn read_i64(&mut self) -> Result<i64>;
    fn read_f32(&mut self) -> Result<f32>;
    fn read_f64(&mut self) -> Result<f64>;

    fn read_u8_into(&mut self, dest: &mut [u8]) -> Result<()>;
    fn read_i8_into(&mut self, dest: &mut [i8]) -> Result<()>;
    fn read_u16_into(&mut self, dest: &mut [u16]) -> Result<()>;
    fn read_i16_into(&mut self, dest: &mut [i16]) -> Result<()>;
    fn read_u32_into(&mut self, dest: &mut [u32]) -> Result<()>;
    fn read_i32_into(&mut self, dest: &mut [i32]) -> Result<()>;
    fn read_u64_into(&mut self, dest: &mut [u64]) -> Result<()>;
    fn read_i64_into(&mut self, dest: &mut [i64]) -> Result<()>;
    fn read_f32_into(&mut self, dest: &mut [f32]) -> Result<()>;
    fn read_f64_into(&mut self, dest: &mut [f64]) -> Result<()>;

    fn read_string(&mut self) -> Result<String>;
}

impl<T> Reader for T where T: Read {
    fn read_all(&mut self, buf: &mut Vec<u8>) -> Result<()> {
        let bytes = self.bytes();
        for byte in bytes {
            buf.push(byte?);
        }
        Ok(())
    }

    fn read_u8(&mut self) -> Result<u8> {
        match self.bytes().next() {
            None => Err(TsonError::new("EOF")),
            Some(v) => Ok(v?)
        }
    }

    fn read_i8(&mut self) -> Result<i8> {
        match self.bytes().next() {
            None => Err(TsonError::new("EOF")),
            Some(v) => Ok(v? as i8)
        }
    }

    fn read_u16(&mut self) -> Result<u16> {
        let mut bytes = [0;2];
        self.read_exact(&mut bytes)?;
        Ok(LittleEndian::read_u16(&bytes))
    }

    fn read_i16(&mut self) -> Result<i16> {
        let mut bytes = [0;2];
        self.read_exact(&mut bytes)?;
        Ok(LittleEndian::read_i16(&bytes))
    }

    fn read_u32(&mut self) -> Result<u32> {
        let mut bytes = [0;4];
        self.read_exact(&mut bytes)?;
        Ok(LittleEndian::read_u32(&bytes))
    }

    fn read_i32(&mut self) -> Result<i32> {
        let mut bytes = [0;4];
        self.read_exact(&mut bytes)?;
        Ok(LittleEndian::read_i32(&bytes))
    }

    fn read_u64(&mut self) -> Result<u64> {
        let mut bytes = [0;8];
        self.read_exact(&mut bytes)?;
        Ok(LittleEndian::read_u64(&bytes))
    }

    fn read_i64(&mut self) -> Result<i64> {
        let mut bytes = [0;8];
        self.read_exact(&mut bytes)?;
        Ok(LittleEndian::read_i64(&bytes))
    }

    fn read_f32(&mut self) -> Result<f32> {
        let mut bytes = [0;4];
        self.read_exact(&mut bytes)?;
        Ok(LittleEndian::read_f32(&bytes))
    }

    fn read_f64(&mut self) -> Result<f64> {
        let mut bytes = [0;8];
        self.read_exact(&mut bytes)?;
        Ok(LittleEndian::read_f64(&bytes))
    }

    fn read_u8_into(&mut self, dest: &mut [u8]) -> Result<()> {
        self.read_exact( dest)?;
        Ok(())
    }

    fn read_i8_into(&mut self, dest: &mut [i8]) -> Result<()> {
        let mut bytes = vec![0; dest.len() ];
        self.read_exact(&mut bytes)?;
        bytes.iter().zip(dest.iter_mut()).for_each(|(v, vv)| *vv = (*v) as i8);
        Ok(())
    }

    fn read_u16_into(&mut self, dest: &mut [u16]) -> Result<()> {
        let mut bytes = vec![0; dest.len()*2];
        self.read_exact(&mut bytes)?;
        LittleEndian::read_u16_into(&bytes, dest);
        Ok(())
    }

    fn read_i16_into(&mut self, dest: &mut [i16]) -> Result<()> {
        let mut bytes = vec![0; dest.len()*2];
        self.read_exact(&mut bytes)?;
        LittleEndian::read_i16_into(&bytes, dest);
        Ok(())
    }

    fn read_u32_into(&mut self, dest: &mut [u32]) -> Result<()> {
        let mut bytes = vec![0; dest.len()*4];
        self.read_exact(&mut bytes)?;
        LittleEndian::read_u32_into(&bytes, dest);
        Ok(())
    }

    fn read_i32_into(&mut self, dest: &mut [i32]) -> Result<()> {
        let mut bytes = vec![0; dest.len()*4];
        self.read_exact(&mut bytes)?;
        LittleEndian::read_i32_into(&bytes, dest);
        Ok(())
    }

    fn read_u64_into(&mut self, dest: &mut [u64]) -> Result<()> {
        let mut bytes = vec![0; dest.len()*8];
        self.read_exact(&mut bytes)?;
        LittleEndian::read_u64_into(&bytes, dest);
        Ok(())
    }

    fn read_i64_into(&mut self, dest: &mut [i64]) -> Result<()> {
        let mut bytes = vec![0; dest.len()*8];
        self.read_exact(&mut bytes)?;
        LittleEndian::read_i64_into(&bytes, dest);
        Ok(())
    }

    fn read_f32_into(&mut self, dest: &mut [f32]) -> Result<()> {
        let mut bytes = vec![0; dest.len()*4];
        self.read_exact(&mut bytes)?;
        LittleEndian::read_f32_into(&bytes, dest);
        Ok(())
    }

    fn read_f64_into(&mut self, dest: &mut [f64]) -> Result<()> {
        let mut bytes = vec![0; dest.len()*8];
        self.read_exact(&mut bytes)?;
        LittleEndian::read_f64_into(&bytes, dest);
        Ok(())
    }

    fn read_string(&mut self) -> Result<String> {

        let mut done = false;
        let mut vec = Vec::new();
        while !done {
            let byte = self.read_u8()?;
            if byte == 0 {
                done = true;
            } else {
                vec.push(byte);
            }
        }

        if let Ok(value) = String::from_utf8(vec) {
            Ok(value)
        } else {
            Err(TsonError::new("bad string"))
        }

    }
}

// impl<T> Reader for T where T: Buf {
//     fn read_all(&mut self, buf: &mut Vec<u8>) -> Result<()>{
//         buf.extend(self.iter());
//         Ok(())
//     }
//
//     fn read_u8(&mut self) -> Result<u8> {
//         if self.remaining() < 1 {
//             return Err(TsonError::new("EOF"));
//         }
//         Ok(self.get_u8())
//     }
//     fn read_i8(&mut self) -> Result<i8>{
//         if self.remaining() < 1 {
//             return Err(TsonError::new("EOF"));
//         }
//         Ok(self.get_i8())
//     }
//     fn read_u16(&mut self) -> Result<u16>{
//         if self.remaining() < 2 {
//             return Err(TsonError::new("EOF"));
//         }
//         Ok(self.get_u16_le())
//     }
//     fn read_i16(&mut self) -> Result<i16>{
//         if self.remaining() < 2 {
//             return Err(TsonError::new("EOF"));
//         }
//         Ok(self.get_i16_le())
//     }
//     fn read_u32(&mut self) -> Result<u32>{
//         if self.remaining() < 4 {
//             return Err(TsonError::new("EOF"));
//         }
//         Ok(self.get_u32_le())
//     }
//     fn read_i32(&mut self) -> Result<i32>{
//         if self.remaining() < 4 {
//             return Err(TsonError::new("EOF"));
//         }
//         Ok(self.get_i32_le())
//     }
//     fn read_u64(&mut self) -> Result<u64>{
//         if self.remaining() < 8 {
//             return Err(TsonError::new("EOF"));
//         }
//         Ok(self.get_u64_le())
//     }
//     fn read_i64(&mut self) -> Result<i64>{
//         if self.remaining() < 8 {
//             return Err(TsonError::new("EOF"));
//         }
//         Ok(self.get_i64_le())
//     }
//     fn read_f32(&mut self) -> Result<f32>{
//         if self.remaining() < 4 {
//             return Err(TsonError::new("EOF"));
//         }
//         Ok(self.get_f32_le())
//     }
//     fn read_f64(&mut self) -> Result<f64>{
//         if self.remaining() < 8 {
//             return Err(TsonError::new("EOF"));
//         }
//         Ok(self.get_f64_le())
//     }
// }


pub struct Deserializer {}

impl Deserializer {
    pub fn new() -> Deserializer { Deserializer {} }

    pub fn read(&self, reader: &mut dyn Reader) -> Result<Value> {
        let itype = self.read_type(reader)?;

        if itype != STRING_TYPE {
            return Err(TsonError::new("wrong format -- expect version as str"));
        }

        let version = self.read_string(reader)?;

        if !version.eq(VERSION) {
            return Err(TsonError::new("wrong version"));
        }

        self.read_object(reader)
    }

    fn read_type(&self, reader: &mut dyn Reader) -> Result<u8> {
        reader.read_u8()
    }

    fn read_len(&self, reader: &mut dyn Reader) -> Result<usize> {
        Ok(reader.read_u32()? as usize)
    }

    fn read_string(&self, reader: &mut dyn Reader) -> Result<String> {
        reader.read_string()
    }

    fn read_object(&self, reader: &mut dyn Reader) -> Result<Value> {
        let itype = self.read_type(reader)?;
        match itype {
            NULL_TYPE => Ok(Value::NULL),
            STRING_TYPE => Ok(Value::STR(self.read_string(reader)?)),
            INTEGER_TYPE => {
                Ok(Value::I32(reader.read_i32()?))
            }
            DOUBLE_TYPE => {
                Ok(Value::F64(reader.read_f64()?))
            }
            BOOL_TYPE => {
                Ok(Value::BOOL(reader.read_u8()? > 0))
            }
            LIST_TYPE => {
                let len = self.read_len(reader)?;
                let mut vec = Vec::with_capacity(len);
                for _ in 0..len {
                    vec.push(self.read_object(reader)?);
                }
                Ok(Value::LST(vec))
            }
            MAP_TYPE => {
                let len = self.read_len(reader)?;
                let mut map = HashMap::with_capacity(len);
                for _ in 0..len {
                    if let Value::STR(k) = self.read_object(reader)? {
                        map.insert(k, self.read_object(reader)?);
                    } else {
                        return Err(TsonError::new("wrong format -- MAP_TYPE -- expected STR"));
                    }
                }
                Ok(Value::MAP(map))
            }
            LIST_UINT8_TYPE => {
                let len = self.read_len(reader)?;
                let mut vec = vec![0; len];
                reader.read_u8_into(&mut vec)?;
                Ok(Value::LSTU8(vec))
            }
            LIST_INT8_TYPE => {
                let len = self.read_len(reader)?;
                let mut vec = vec![0; len];
                reader.read_i8_into(&mut vec)?;
                Ok(Value::LSTI8(vec))
            }
            LIST_UINT16_TYPE => {
                let len = self.read_len(reader)?;
                let mut vec = vec![0; len];
                reader.read_u16_into(&mut vec)?;
                Ok(Value::LSTU16(vec))
            }
            LIST_INT16_TYPE => {
                let len = self.read_len(reader)?;
                let mut vec = vec![0; len];
                reader.read_i16_into(&mut vec)?;
                Ok(Value::LSTI16(vec))
            }

            LIST_UINT32_TYPE => {
                let len = self.read_len(reader)?;
                let mut vec = vec![0; len];
                reader.read_u32_into(&mut vec)?;
                Ok(Value::LSTU32(vec))
            }
            LIST_INT32_TYPE => {
                let len = self.read_len(reader)?;
                let mut vec = vec![0; len];
                reader.read_i32_into(&mut vec)?;
                Ok(Value::LSTI32(vec))
            }
            LIST_INT64_TYPE => {
                let len = self.read_len(reader)?;
                let mut vec = vec![0; len];
                reader.read_i64_into(&mut vec)?;
                Ok(Value::LSTI64(vec))
            }
            LIST_UINT64_TYPE => {
                let len = self.read_len(reader)?;
                let mut vec = vec![0; len];
                reader.read_u64_into(&mut vec)?;
                Ok(Value::LSTU64(vec))
            }
            LIST_FLOAT32_TYPE => {
                let len = self.read_len(reader)?;
                let mut vec = vec![0.0; len];
                reader.read_f32_into(&mut vec)?;
                Ok(Value::LSTF32(vec))
            }
            LIST_FLOAT64_TYPE => {
                let len = self.read_len(reader)?;
                let mut vec = vec![0.0; len];
                reader.read_f64_into(&mut vec)?;
                Ok(Value::LSTF64(vec))
            }
            LIST_STRING_TYPE => {
                let mut len_in_bytes = self.read_len(reader)?;

                let mut vec = Vec::new();
                while len_in_bytes > 0 {
                    let v = self.read_string(reader)?;
                    len_in_bytes -= v.as_bytes().len() + 1;
                    vec.push(v);
                }

                if len_in_bytes > 0 {
                    return Err(TsonError::new("LIST_STRING_TYPE -- wrong format"));
                }

                Ok(Value::LSTSTR(vec.into()))
            }

            _ => Err(TsonError::new("wrong format -- _")),
        }
    }
}