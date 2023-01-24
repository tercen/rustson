use ::{Reader, TsonResult};
use ::{MAP_TYPE, TsonError};
use ::{STRING_TYPE, VERSION};
use ::{LIST_TYPE, Value};
use Deserializer;

pub struct TsonGDeserializer {
    reader: Box<dyn Reader>
}

pub struct TsonMapDeser<'a> {
    deser: &'a mut TsonGDeserializer,
    len: usize,
    current: usize,
}

pub struct TsonMapEntryDeser<'a> {
    deser: &'a mut TsonGDeserializer,
    key: String,
}

pub struct TsonListDeser<'a> {
    deser: &'a mut TsonGDeserializer,
    len: usize,
    current: usize,
}

pub struct TsonTypedListDeser<'a> {
    deser: &'a mut TsonGDeserializer,
}

impl<'a> TsonGDeserializer {

    pub fn new(mut reader: Box<dyn Reader>) -> TsonResult<Self> {
        let itype = reader.read_u8()?;

        if itype != STRING_TYPE {
            return Err(TsonError::new("wrong format -- expect version as str"));
        }

        let version = reader.read_string()?;

        if !version.eq(VERSION) {
            return Err(TsonError::new("wrong version"));
        }

        Ok(TsonGDeserializer {reader})
    }

    pub fn read_type(&mut self) -> TsonResult<u8> {
        self.reader.read_u8()
    }

    pub fn read_len(&mut self) -> TsonResult<usize> {
        Ok(self.reader.read_u32()? as usize)
    }

    pub fn next_string(&mut self) -> TsonResult<String> {
        if self.read_type()? == STRING_TYPE {
            self.reader.read_string()
        } else {
            Err(TsonError::new("TsonDeser -- bad type -- String expected"))
        }
    }

    pub fn next_map(&mut self) -> TsonResult<TsonMapDeser> {
        let t = self.read_type()?;
        if t == MAP_TYPE {
            TsonMapDeser::new(self)
        } else {
            Err(TsonError::new(format!("bad type -- MAP expected -- found {}", t)))
        }

    }

    pub fn next_list(&mut self) -> TsonResult<TsonListDeser> {
        if self.read_type()? == LIST_TYPE {
            TsonListDeser::new(self)
        } else {
            Err(TsonError::new("bad type -- LIST expected"))
        }
    }

    pub fn next_value(&mut self) -> TsonResult<Value> {
        Deserializer::new().read_object(self.reader.as_mut())
    }
}

impl<'a> TsonListDeser<'a> {
    pub fn new(deser: &'a mut TsonGDeserializer) -> TsonResult<Self> {
        let len = deser.read_len()?;
        Ok(TsonListDeser{deser, len, current: 0 })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn next_map(&mut self) -> Option<TsonResult<TsonMapDeser>> {
        if self.current < self.len() {
            self.current += 1;
            Some(self.deser.next_map())
        } else {
            None
        }
    }

    pub fn next_list(&mut self) -> Option<TsonResult<TsonListDeser>> {
        if self.current < self.len() {
            self.current += 1;
            Some(self.deser.next_list())
        } else {
            None
        }
    }

    pub fn value(&mut self) -> Option<TsonResult<Value>> {
        if self.current < self.len() {
            self.current += 1;
            Some(self.deser.next_value())
        } else {
            None
        }
    }
}

impl<'a> TsonMapDeser<'a> {
    pub fn new(deser: &'a mut TsonGDeserializer) -> TsonResult<Self> {
        let len = deser.read_len()?;
        Ok(TsonMapDeser{deser, len, current: 0 })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn next(&mut self) -> Option<TsonResult<TsonMapEntryDeser>> {
        if self.current < self.len() {
            self.current += 1;
            Some(TsonMapEntryDeser::new(  &mut self.deser))
        } else {
            None
        }
    }
}

impl<'a> TsonMapEntryDeser<'a> {
    pub fn new(deser: &'a mut TsonGDeserializer) -> TsonResult<Self> {
        let key = deser.next_string()?;
        Ok(TsonMapEntryDeser{deser, key})
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn string(&mut self) -> TsonResult<String> {
        self.deser.next_string()
    }

    pub fn map(&mut self) -> TsonResult<TsonMapDeser> {
        self.deser.next_map()
    }

    pub fn list(&mut self) -> TsonResult<TsonListDeser> {
        self.deser.next_list()
    }

    pub fn value(&mut self) -> TsonResult<Value> {
        self.deser.next_value()
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::io::Cursor;
    use ::{encode, Value};
    use gdeser::{TsonGDeserializer, TsonMapEntryDeser};
    use TsonResult;

    #[test]
    fn next_map() -> TsonResult<()>{
        let mut map = HashMap::new();
        map.insert("name".to_string(), Value::STR("factor1".to_string()));
        map.insert("values".to_string(), Value::LSTF64(vec![0.0,42.0]));
        let bytes = encode(&Value::MAP(map)).unwrap();
        let mut reader = Box::new(Cursor::new(bytes));

        let mut deser = TsonGDeserializer::new(reader)?;

        let mut deser_map = deser.next_map()?;

        assert_eq!(deser_map.len(), 2);


        fn check_key_value(key_value: &mut TsonMapEntryDeser) -> TsonResult<()> {
            if &key_value.key == "name" {
                assert_eq!(key_value.string()?, "factor1");
            } else if &key_value.key == "values" {
                if let Value::LSTF64(vec) = key_value.value()? {
                    assert_eq!(vec, vec![0.0,42.0]);
                } else {
                    assert!(false)
                }
            }
            Ok(())
        }

        check_key_value(&mut deser_map.next().unwrap()?)?;
        check_key_value(&mut deser_map.next().unwrap()?)?;

        Ok(())
    }

    #[test]
    fn next_list() -> TsonResult<()>{
        let mut map1 = HashMap::new();
        map1.insert("name".to_string(), Value::STR("factor1".to_string()));
        map1.insert("values".to_string(), Value::LSTF64(vec![0.0,42.0]));

        let mut map2 = HashMap::new();
        map2.insert("name".to_string(), Value::STR("factor2".to_string()));
        map2.insert("values".to_string(), Value::LSTF64(vec![0.0,42.0]));

        let bytes = encode(&Value::LST(vec![Value::MAP(map1), Value::MAP(map2)])).unwrap();
        let mut reader = Box::new(Cursor::new(bytes));
        let mut deser = TsonGDeserializer::new(reader)?;

        let mut deser_list = deser.next_list()?;

        assert_eq!(deser_list.len(), 2);

        fn check_key_value(key_value: &mut TsonMapEntryDeser) -> TsonResult<()> {
            if &key_value.key == "name" {
                assert!(key_value.string()?.starts_with("factor"));
            } else if &key_value.key == "values" {
                if let Value::LSTF64(vec) = key_value.value()? {
                    assert_eq!(vec, vec![0.0,42.0]);
                } else {
                    assert!(false)
                }
            }
            Ok(())
        }

        let mut map = deser_list.next_map().unwrap()?;
        check_key_value(&mut map.next().unwrap()?)?;
        check_key_value(&mut map.next().unwrap()?)?;

        let mut map = deser_list.next_map().unwrap()?;
        check_key_value(&mut map.next().unwrap()?)?;
        check_key_value(&mut map.next().unwrap()?)?;

        Ok(())
    }
}