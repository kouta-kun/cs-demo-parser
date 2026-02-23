use std::fmt::{Debug, Formatter};
use source2_demo::proto::c_msg_source1_legacy_game_event::KeyT;
use source2_demo::FieldValue;

pub enum Variant {
    String(String),
    F32(f32),
    I32(i32),
    U32(u32),
    U64(u64),
    Bool(bool),
}

impl Debug for Variant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_string().as_str())
    }
}

impl Variant {
    fn to_string(&self) -> String {
        match self {
            Variant::String(str) => str.clone(),
            Variant::F32(f32) => f32.to_string(),
            Variant::I32(i32) => i32.to_string(),
            Variant::U32(u32) => u32.to_string(),
            Variant::U64(u64) => u64.to_string(),
            Variant::Bool(bool) => bool.to_string(),
        }
    }

    pub fn i32(&self) -> i32 {
        let i32 = match self {
            Variant::I32(i32) => i32,
            _ => panic!("not i32."),
        };
        *i32
    }

    pub fn u32(&self) -> u32 {
        let u32 = match self {
            Variant::U32(u32) => u32,
            _ => panic!("not u32."),
        };
        *u32
    }

    pub fn u64(&self) -> u64 {
        let u64 = match self {
            Variant::U64(u64) => u64,
            _ => panic!("not u64."),
        };
        *u64
    }

    pub fn string(&self) -> String {
        let string = match self {
            Variant::String(str) => str,
            _ => panic!("not string."),
        };
        string.clone()
    }

    pub fn bool(&self) -> bool {
        let bool = match self {
            Variant::Bool(bool) => bool,
            _ => panic!("not bool."),
        };
        *bool
    }

    pub fn f32(&self) -> f32 {
        let f32 = match self {
            Variant::F32(f32) => f32,
            _ => panic!("not bool."),
        };
        *f32
    }
}

impl OptionFrom<&FieldValue> for Variant {
    fn opt_from(t: &FieldValue) -> Option<Self> {
        match t {
            FieldValue::Boolean(b) => Some(Variant::Bool(*b)),
            FieldValue::String(s) => Some(Variant::String(s.clone())),
            FieldValue::Float(f) => Some(Variant::F32(*f)),
            FieldValue::Vector2D(_) => None,
            FieldValue::Vector3D(_) => None,
            FieldValue::Vector4D(_) => None,
            FieldValue::Signed8(_) => None,
            FieldValue::Signed16(_) => None,
            FieldValue::Signed32(i32) => Some(Variant::I32(*i32)),
            FieldValue::Signed64(_) => None,
            FieldValue::Unsigned8(_) => None,
            FieldValue::Unsigned16(_) => None,
            FieldValue::Unsigned32(u32) => Some(Variant::U32(*u32)),
            FieldValue::Unsigned64(u64) => Some(Variant::U64(*u64)),
        }
    }
}

impl OptionFrom<&KeyT> for Variant {
    fn opt_from(key: &KeyT) -> Option<Self> {
        match key.r#type {
            None => {
                None
            }
            Some(r#type) => {
                match r#type {
                    1 => Some(Variant::String(key.val_string().to_owned())),
                    2 => Some(Variant::F32(key.val_float())),
                    // These seem to return an i32
                    3 => Some(Variant::I32(key.val_long())),
                    4 => Some(Variant::I32(key.val_short().try_into().unwrap_or(-1))),
                    5 => Some(Variant::I32(key.val_byte().try_into().unwrap_or(-1))),
                    6 => Some(Variant::Bool(key.val_bool())),
                    7 => Some(Variant::U64(key.val_uint64())),
                    8 => Some(Variant::I32(key.val_long().try_into().unwrap_or(-1))),
                    9 => Some(Variant::I32(key.val_short().try_into().unwrap_or(-1))),
                    _ => panic!("Unknown type"),
                }
            }
        }
    }
}

pub trait OptionFrom<T> where Self: Sized {
    fn opt_from(t: T) -> Option<Self>;
}