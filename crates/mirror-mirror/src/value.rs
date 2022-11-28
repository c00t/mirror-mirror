use serde::{Deserialize, Serialize};
use speedy::{Readable, Writable};
use std::{any::Any, fmt};

use crate::{
    tuple::TupleValue, Enum, EnumValue, FromReflect, List, Reflect, Struct, StructValue, Tuple,
    TupleStruct, TupleStructValue,
};

#[derive(Readable, Writable, Serialize, Deserialize, Debug, Clone)]
pub struct Value {
    data: ValueData,
}

impl Value {
    pub fn new(data: ValueData) -> Self {
        Self { data }
    }
}

impl Reflect for Value {
    fn as_any(&self) -> &dyn Any {
        self.data.as_any()
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self.data.as_any_mut()
    }

    fn as_reflect(&self) -> &dyn Reflect {
        self.data.as_reflect()
    }

    fn as_reflect_mut(&mut self) -> &mut dyn Reflect {
        self.data.as_reflect_mut()
    }

    fn as_tuple(&self) -> Option<&dyn Tuple> {
        self.data.as_tuple()
    }

    fn as_tuple_mut(&mut self) -> Option<&mut dyn Tuple> {
        self.data.as_tuple_mut()
    }

    fn as_struct(&self) -> Option<&dyn Struct> {
        self.data.as_struct()
    }

    fn as_struct_mut(&mut self) -> Option<&mut dyn Struct> {
        self.data.as_struct_mut()
    }

    fn as_tuple_struct(&self) -> Option<&dyn TupleStruct> {
        self.data.as_tuple_struct()
    }

    fn as_tuple_struct_mut(&mut self) -> Option<&mut dyn TupleStruct> {
        self.data.as_tuple_struct_mut()
    }

    fn as_enum(&self) -> Option<&dyn Enum> {
        self.data.as_enum()
    }

    fn as_enum_mut(&mut self) -> Option<&mut dyn Enum> {
        self.data.as_enum_mut()
    }

    fn as_list(&self) -> Option<&dyn List> {
        self.data.as_list()
    }

    fn as_list_mut(&mut self) -> Option<&mut dyn List> {
        self.data.as_list_mut()
    }

    fn patch(&mut self, value: &dyn Reflect) {
        self.data.patch(value);
    }

    fn to_value(&self) -> Value {
        self.data.to_value()
    }

    fn clone_reflect(&self) -> Box<dyn Reflect> {
        Box::new(self.clone())
    }

    fn debug(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "{:#?}", self)
        } else {
            write!(f, "{:?}", self)
        }
    }
}

impl FromReflect for Value {
    fn from_reflect(reflect: &dyn Reflect) -> Option<Self> {
        Some(reflect.to_value())
    }
}

#[non_exhaustive]
#[allow(non_camel_case_types)]
#[derive(Readable, Writable, Serialize, Deserialize, Debug, Clone)]
pub enum ValueData {
    usize(usize),
    u8(u8),
    u16(u16),
    u32(u32),
    u64(u64),
    u128(u128),
    i8(i8),
    i16(i16),
    i32(i32),
    i64(i64),
    i128(i128),
    bool(bool),
    char(char),
    f32(f32),
    f64(f64),
    String(String),
    StructValue(Box<StructValue>),
    EnumValue(Box<EnumValue>),
    TupleStructValue(TupleStructValue),
    TupleValue(TupleValue),
    List(Vec<Value>),
}

impl Reflect for ValueData {
    fn as_any(&self) -> &dyn Any {
        match self {
            ValueData::usize(inner) => inner,
            ValueData::u8(inner) => inner,
            ValueData::u16(inner) => inner,
            ValueData::u32(inner) => inner,
            ValueData::u64(inner) => inner,
            ValueData::u128(inner) => inner,
            ValueData::i8(inner) => inner,
            ValueData::i16(inner) => inner,
            ValueData::i32(inner) => inner,
            ValueData::i64(inner) => inner,
            ValueData::i128(inner) => inner,
            ValueData::bool(inner) => inner,
            ValueData::char(inner) => inner,
            ValueData::f32(inner) => inner,
            ValueData::f64(inner) => inner,
            ValueData::String(inner) => inner,
            ValueData::StructValue(inner) => inner,
            ValueData::TupleStructValue(inner) => inner,
            ValueData::EnumValue(inner) => inner,
            ValueData::TupleValue(inner) => inner,
            ValueData::List(inner) => inner,
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        match self {
            ValueData::usize(inner) => inner,
            ValueData::u8(inner) => inner,
            ValueData::u16(inner) => inner,
            ValueData::u32(inner) => inner,
            ValueData::u64(inner) => inner,
            ValueData::u128(inner) => inner,
            ValueData::i8(inner) => inner,
            ValueData::i16(inner) => inner,
            ValueData::i32(inner) => inner,
            ValueData::i64(inner) => inner,
            ValueData::i128(inner) => inner,
            ValueData::bool(inner) => inner,
            ValueData::char(inner) => inner,
            ValueData::f32(inner) => inner,
            ValueData::f64(inner) => inner,
            ValueData::String(inner) => inner,
            ValueData::StructValue(inner) => inner,
            ValueData::TupleStructValue(inner) => inner,
            ValueData::EnumValue(inner) => inner,
            ValueData::TupleValue(inner) => inner,
            ValueData::List(inner) => inner,
        }
    }

    fn as_reflect(&self) -> &dyn Reflect {
        self
    }

    fn as_reflect_mut(&mut self) -> &mut dyn Reflect {
        self
    }

    fn patch(&mut self, value: &dyn Reflect) {
        match self {
            ValueData::usize(inner) => {
                if let Some(value) = value.downcast_ref::<usize>() {
                    *inner = *value;
                }
            }
            ValueData::u8(inner) => {
                if let Some(value) = value.downcast_ref::<u8>() {
                    *inner = *value;
                }
            }
            ValueData::u16(inner) => {
                if let Some(value) = value.downcast_ref::<u16>() {
                    *inner = *value;
                }
            }
            ValueData::u32(inner) => {
                if let Some(value) = value.downcast_ref::<u32>() {
                    *inner = *value;
                }
            }
            ValueData::u64(inner) => {
                if let Some(value) = value.downcast_ref::<u64>() {
                    *inner = *value;
                }
            }
            ValueData::u128(inner) => {
                if let Some(value) = value.downcast_ref::<u128>() {
                    *inner = *value;
                }
            }
            ValueData::i8(inner) => {
                if let Some(value) = value.downcast_ref::<i8>() {
                    *inner = *value;
                }
            }
            ValueData::i16(inner) => {
                if let Some(value) = value.downcast_ref::<i16>() {
                    *inner = *value;
                }
            }
            ValueData::i32(inner) => {
                if let Some(value) = value.downcast_ref::<i32>() {
                    *inner = *value;
                }
            }
            ValueData::i64(inner) => {
                if let Some(value) = value.downcast_ref::<i64>() {
                    *inner = *value;
                }
            }
            ValueData::i128(inner) => {
                if let Some(value) = value.downcast_ref::<i128>() {
                    *inner = *value;
                }
            }
            ValueData::bool(inner) => {
                if let Some(value) = value.downcast_ref::<bool>() {
                    *inner = *value;
                }
            }
            ValueData::char(inner) => {
                if let Some(value) = value.downcast_ref::<char>() {
                    *inner = *value;
                }
            }
            ValueData::f32(inner) => {
                if let Some(value) = value.downcast_ref::<f32>() {
                    *inner = *value;
                }
            }
            ValueData::f64(inner) => {
                if let Some(value) = value.downcast_ref::<f64>() {
                    *inner = *value;
                }
            }
            ValueData::String(inner) => {
                if let Some(value) = value.downcast_ref::<String>() {
                    *inner = value.clone();
                }
            }
            ValueData::StructValue(inner) => {
                if let Some(value) = value.downcast_ref::<StructValue>() {
                    *inner = Box::new(value.clone());
                }
            }
            ValueData::TupleStructValue(inner) => {
                if let Some(value) = value.downcast_ref::<TupleStructValue>() {
                    *inner = value.clone();
                }
            }
            ValueData::EnumValue(inner) => {
                if let Some(value) = value.downcast_ref::<EnumValue>() {
                    *inner = Box::new(value.clone());
                }
            }
            ValueData::TupleValue(inner) => {
                if let Some(value) = value.downcast_ref::<TupleValue>() {
                    *inner = value.clone();
                }
            }
            ValueData::List(inner) => {
                if let Some(value) = value.downcast_ref::<Vec<Value>>() {
                    *inner = value.clone();
                }
            }
        }
    }

    fn to_value(&self) -> Value {
        Value::new(self.clone())
    }

    fn clone_reflect(&self) -> Box<dyn Reflect> {
        Box::new(self.clone())
    }

    fn as_tuple(&self) -> Option<&dyn Tuple> {
        if let ValueData::TupleValue(value) = self {
            Some(value)
        } else {
            None
        }
    }

    fn as_tuple_mut(&mut self) -> Option<&mut dyn Tuple> {
        if let ValueData::TupleValue(value) = self {
            Some(&mut *value)
        } else {
            None
        }
    }

    fn as_struct(&self) -> Option<&dyn Struct> {
        if let ValueData::StructValue(value) = self {
            Some(&**value)
        } else {
            None
        }
    }

    fn as_struct_mut(&mut self) -> Option<&mut dyn Struct> {
        if let ValueData::StructValue(value) = self {
            Some(&mut **value)
        } else {
            None
        }
    }

    fn as_tuple_struct(&self) -> Option<&dyn TupleStruct> {
        if let ValueData::TupleStructValue(value) = self {
            Some(value)
        } else {
            None
        }
    }

    fn as_tuple_struct_mut(&mut self) -> Option<&mut dyn TupleStruct> {
        if let ValueData::TupleStructValue(value) = self {
            Some(value)
        } else {
            None
        }
    }

    fn as_enum(&self) -> Option<&dyn Enum> {
        if let ValueData::EnumValue(value) = self {
            Some(&**value)
        } else {
            None
        }
    }

    fn as_enum_mut(&mut self) -> Option<&mut dyn Enum> {
        if let ValueData::EnumValue(value) = self {
            Some(&mut **value)
        } else {
            None
        }
    }

    fn as_list(&self) -> Option<&dyn List> {
        if let ValueData::List(value) = self {
            Some(value)
        } else {
            None
        }
    }

    fn as_list_mut(&mut self) -> Option<&mut dyn List> {
        if let ValueData::List(value) = self {
            Some(&mut *value)
        } else {
            None
        }
    }

    fn debug(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "{:#?}", self)
        } else {
            write!(f, "{:?}", self)
        }
    }
}

macro_rules! from_impls {
    (
        $($ident:ident)*
    ) => {
        $(
            impl From<$ident> for ValueData {
                fn from(value: $ident) -> Self {
                    ValueData::$ident(value)
                }
            }

            impl From<$ident> for Value {
                fn from(value: $ident) -> Self {
                    Value::new(ValueData::from(value))
                }
            }
        )*
    };
}

impl From<StructValue> for Value {
    fn from(value: StructValue) -> Self {
        Value::new(ValueData::StructValue(Box::new(value)))
    }
}

impl From<EnumValue> for Value {
    fn from(value: EnumValue) -> Self {
        Value::new(ValueData::EnumValue(Box::new(value)))
    }
}

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self {
        Value::new(ValueData::List(value))
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::new(value.to_owned().into())
    }
}

from_impls! {
    usize u8 u16 u32 u64 u128
    i8 i16 i32 i64 i128
    f32 f64
    bool char String
    TupleValue TupleStructValue
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn has_small_stack_size() {
        // if we can get the value to be smaller than 32 then that'd be cool
        // but 32 is probably also fine
        assert_eq!(std::mem::size_of::<ValueData>(), 32);
    }
}
