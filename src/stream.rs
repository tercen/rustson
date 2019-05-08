use super::*;
use futures::{Stream, Poll, Async};
use ser2::*;

struct SerializerStream<'v> {
    ser: Serializer2<'v>,
}

impl<'v> SerializerStream<'v> {
    pub fn new(max_buf_len: usize, value: &'v Value) -> SerializerStream<'v> {
        SerializerStream { ser: Serializer2::new(max_buf_len, &value) }
    }
}

impl<'v> Stream for SerializerStream<'v> {
    type Item = Vec<u8>;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let mut buf = Vec::new();
        match self.ser.write(&mut buf) {
            Ok(_) => {
                if buf.is_empty() {
                    Ok(Async::Ready(None))
                } else {
                    Ok(Async::Ready(Some(buf)))
                }
            }
            Err(_e) => Err(())
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn encode_decode(object: &Value) {
        let mut bytes = Vec::new();
        let mut buf = Vec::new();

        let mut done = false;
        let mut ser = Serializer2::new(10, object);

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
        assert_eq!(object, &value);
    }

    #[test]
    fn serializer_stream() {
        // cargo test serializer_stream -- --nocapture

        use tokio_core::reactor::Core;
        use futures::future::lazy;

        use tokio::runtime::Runtime;

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

        let mut map = HashMap::new();
        map.insert("i42".to_owned(), Value::I32(42));

        let mut inner_map = HashMap::new();
        inner_map.insert("u42".to_owned(), Value::LSTU8(vec![42]));

        map.insert("map".to_owned(), Value::MAP(inner_map));

        vec.push(Value::MAP(map));

        let object = Value::LST(vec);

        let stream = SerializerStream::new(12, &object);

        let fut = stream.for_each(|value| {
            println!("{:#?}", value);
            Ok(())
        });

        let mut core = Core::new().unwrap();

        core.run(fut);
//

//        let fut = lazy(move || {
//            let object = Value::LST(vec);
//
//            let stream = SerializerStream::new(12, &object);
//
//            stream.for_each(|value| {
//                println!("{:#?}", value);
//                Ok(())
//            })
//        });
//
//        tokio::run(fut);


    }
}
