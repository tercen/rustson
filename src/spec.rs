pub trait TsonTypedListType {
    fn to_int32() -> u8;
}

impl TsonTypedListType for u8 {
    fn to_int32() -> u8 {
        LIST_UINT8_TYPE
    }
}

impl TsonTypedListType for i8 {
    fn to_int32() -> u8 {
        LIST_INT8_TYPE
    }
}

impl TsonTypedListType for u16 {
    fn to_int32() -> u8 {
        LIST_UINT16_TYPE
    }
}

impl TsonTypedListType for i16 {
    fn to_int32() -> u8 {
        LIST_INT16_TYPE
    }
}

impl TsonTypedListType for u32 {
    fn to_int32() -> u8 {
        LIST_UINT32_TYPE
    }
}

impl TsonTypedListType for i32 {
    fn to_int32() -> u8 {
        LIST_INT32_TYPE
    }
}

impl TsonTypedListType for u64 {
    fn to_int32() -> u8 {
        LIST_UINT64_TYPE
    }
}

impl TsonTypedListType for i64 {
    fn to_int32() -> u8 {
        LIST_INT64_TYPE
    }
}

impl TsonTypedListType for f32 {
    fn to_int32() -> u8 {
        LIST_FLOAT32_TYPE
    }
}

impl TsonTypedListType for f64 {
    fn to_int32() -> u8 {
        LIST_FLOAT64_TYPE
    }
}

pub const NULL_TYPE: u8 = 0;
pub const STRING_TYPE: u8 = 1;
pub const INTEGER_TYPE: u8 = 2;
pub const DOUBLE_TYPE: u8 = 3;
pub const BOOL_TYPE: u8 = 4;

pub const LIST_TYPE: u8 = 10;
pub const MAP_TYPE: u8 = 11;

pub const MIN_LIST_TYPED: u8 = LIST_UINT8_TYPE;
pub const MAX_LIST_TYPED: u8 = LIST_FLOAT64_TYPE;

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