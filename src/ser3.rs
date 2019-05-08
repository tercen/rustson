extern crate bytes;

use super::*;

use ser2::ValueTypeLenSize;
use std::ops::Range;
use bytes::BufMut;


#[derive(Debug)]
struct SerializerStack {
    value: Value,
    range: Option<Range<usize>>,
    value_info: ValueTypeLenSize,
}

impl SerializerStack {
    fn new(value: Value) -> SerializerStack {
        let info = ValueTypeLenSize::from_value(&value);
        SerializerStack { value, range: None, value_info: info }
    }
}

#[derive(Debug)]
struct Serializer3 {
    stack: Vec<SerializerStack>,
    max_buf_len: usize,
    is_first_call: bool,
}

impl Serializer3 {
    pub fn new(max_buf_len: usize, value: Value) -> Serializer3 {
        let mut stack = Vec::new();
        stack.push(SerializerStack::new(value));
        Serializer3 { max_buf_len, stack, is_first_call: true }
    }

    pub fn write(&mut self, buf: &mut Vec<u8>) -> Result<()> {
        if self.is_first_call {
            self.is_first_call = false;
            self.add_string(buf, VERSION);
        }
        while !self.stack.is_empty() && buf.len() < self.max_buf_len {
            match self.stack.pop() {
                Some(mut stack_value) => {
                    if stack_value.range.is_none() {
                        buf.put_u8(stack_value.value_info.value_type);

                        if stack_value.value_info.len.is_some() {
                            match stack_value.value {
                                Value::LSTSTR(ref v) => {
                                    let mut len_in_bytes = 0;
                                    for i in v.iter() {
                                        len_in_bytes += i.as_bytes().len() + 1;
                                    }

                                    self.add_len(buf, len_in_bytes)?;
                                }
                                _ => {
                                    self.add_len(buf, stack_value.value_info.len.unwrap())?;
                                }
                            }
                        }
                    }

                    let range = self.get_range(buf.len(), &stack_value);

                    stack_value.range = Some(range.clone());

                    let range_end = range.end;

                    match stack_value.value {
                        Value::NULL => {}
                        Value::STR(ref v) => {
                            self.add_cstring(buf, v);
                        }
                        Value::I32(v) => {
                            buf.put_i32_le(v);
                        }
                        Value::F64(v) => {
                            buf.put_f64_le(v);
                        }
                        Value::BOOL(v) => {
                            buf.put_u8(v as u8);
                        }
                        Value::LSTU8(ref v) => {
                            for i in range { buf.put_u8(v[i]); }

                            if stack_value.value_info.len.is_some() &&
                                range_end < stack_value.value_info.len.unwrap() {
                                self.stack.push(stack_value);
                            }
                        }
                        Value::LSTI8(ref v) => {
                            for i in range { buf.put_i8(v[i]); }
                        }
                        Value::LSTU16(ref v) => {
                            for i in range { buf.put_u16_le(v[i]); }
                        }
                        Value::LSTI16(ref v) => {
                            for i in range { buf.put_i16_le(v[i]); }
                        }
                        Value::LSTU32(ref v) => {
                            for i in range { buf.put_u32_le(v[i]); }
                        }
                        Value::LSTI32(ref v) => {
                            for i in range { buf.put_i32_le(v[i]); }
                        }
                        Value::LSTU64(ref v) => {
                            for i in range { buf.put_u64_le(v[i]); }
                        }
                        Value::LSTI64(ref v) => {
                            for i in range { buf.put_i64_le(v[i]); }
                        }
                        Value::LSTF32(ref v) => {
                            for i in range { buf.put_f32_le(v[i]); }
                        }
                        Value::LSTF64(ref v) => {
                            for i in range { buf.put_f64_le(v[i]); }
                        }
                        Value::LSTSTR(ref v) => {
                            for i in range {
                                self.add_cstring(buf, &v[i]);
                            }
                        }
                        Value::LST(ref mut v) => {
                            for i in range {
                                let value = std::mem::replace(&mut v[i], Value::NULL);
                                self.stack.push(SerializerStack::new(value));
                            }
                        }
                        Value::MAP(ref mut v) => {
                            let mut count: usize = 0;
                            for (k, v) in v.iter_mut() {
                                if range.start <= count && range.end > count {
                                    self.add_string(buf, k);
                                    let value = std::mem::replace(v, Value::NULL);
                                    self.stack.push(SerializerStack::new(value));
                                }
                                count += 1;
                            }
                        }
                    }
                }
                None => {}
            }
        }
        Ok(())
    }


    fn get_range(&self, buf_len: usize, stack_value: &SerializerStack) -> Range<usize> {
        if stack_value.value_info.len.is_none() {
            return Range { start: 0, end: 0 };
        }

        match stack_value.range {
            Some(ref range) => {
                let n_written = range.end;

                if stack_value.value_info.value_size.is_none() {
                    // Value::LST, LSTSTR, MAP
                    return Range { start: n_written, end: n_written + 1 };
                }

                let len = stack_value.value_info.len.unwrap();
                let value_size = stack_value.value_info.value_size.unwrap();

                let n_to_write: usize;
                if self.max_buf_len < buf_len {
                    n_to_write = std::cmp::min(1, len - n_written);
                } else {
                    n_to_write = std::cmp::min(std::cmp::max(1, (self.max_buf_len - buf_len) / value_size as usize),
                                               len - n_written);
                }
                Range { start: n_written, end: n_written + n_to_write }
            }
            None => {
                return Range { start: 0, end: 0 };
            }
        }
    }

    fn add_len(&self, buf: &mut Vec<u8>, len: usize) -> Result<()> {
        if len > MAX_LIST_LENGTH {
            return Err(TsonError::new("list too large"));
        }
        buf.put_u32_le(len as u32);
        Ok(())
    }

    fn add_string(&self, buf: &mut Vec<u8>, value: &str) {
        buf.put_u8(STRING_TYPE);
        self.add_cstring(buf, value);
    }

    fn add_cstring(&self, buf: &mut Vec<u8>, value: &str) {
        for byte in value.as_bytes().iter() {
            buf.put_u8(*byte);
        }
        buf.put_u8(0);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn encode_decode(object: Value) {
        let mut bytes = Vec::new();
        let mut buf = Vec::new();

        let mut done = false;
        let mut ser = Serializer3::new(10, object.clone());

        while !done {
            ser.write(&mut buf).unwrap();
            if buf.is_empty() {
                done = true;
            }
            for byte in buf.iter() {
                bytes.push(*byte);
            }
            buf.clear();
        }

        let value = decode(Cursor::new(&bytes)).unwrap();
        assert_eq!(&object, &value);
    }

    #[test]
    fn serializer3() {
// cargo test serializer3 -- --nocapture
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
        vec.push(Value::LST(vec![Value::LSTF64(vec![42.0; 1000])]));

        vec.push(Value::LSTSTR(vec!["42".to_owned(), "42".to_owned()]));

        let mut map = HashMap::new();
        map.insert("i42".to_owned(), Value::I32(42));

        let mut inner_map = HashMap::new();
        inner_map.insert("u42".to_owned(), Value::LSTU8(vec![42]));

        map.insert("map".to_owned(), Value::MAP(inner_map));

        vec.push(Value::MAP(map));

        let lst = Value::LST(vec);

        encode_decode(lst);
    }
}
