//! Fake implementation for [`uuid::Uuid`]

use fixed_type_id::{prelude::*, type_name, type_id};
use kollect::LinearMap;
use uuid::Uuid;

use crate as mirror_mirror;
use crate::tuple_struct::TupleStructValue;
use crate::TupleStruct;
use crate::__private::ValueIterMut;
use crate::{
    struct_::{FieldsIter, FieldsIterMut, StructValue},
    type_info::graph::*,
    DefaultValue, DescribeType, FromReflect, Reflect, ReflectMut, ReflectOwned, ReflectRef, Struct,
    Value,
};
use std::any::Any;
use std::fmt;

impl DefaultValue for Uuid {
    fn default_value() -> Option<Value> {
        Some(Self::default().to_value())
    }
}

impl DescribeType for Uuid {
    fn build(graph: &mut TypeGraph) -> NodeId {
        let fields = &[UnnamedFieldNode::new::<[u8; 16]>(
            LinearMap::from([]),
            &[],
            graph,
        )];
        graph.get_or_build_node_with::<Self, _>(|graph| {
            TupleStructNode::new::<Self>(fields, LinearMap::from([]), &[])
        })
    }
}

impl Reflect for Uuid {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn as_reflect(&self) -> &dyn Reflect {
        self
    }
    fn as_reflect_mut(&mut self) -> &mut dyn Reflect {
        self
    }
    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
    fn type_name(&self) -> &str {
        self::type_name::<Self>()
    }
    fn type_id(&self) -> self::FixedId {
        self::type_id::<Self>()
    }
    fn patch(&mut self, value: &dyn Reflect) {
        if let Some(tuple_struct) = value.reflect_ref().as_tuple_struct() {
            if let Some(new_value) = tuple_struct.field_at(0usize) {
                self.field_at_mut(0usize).unwrap().patch(new_value);
            }
        }
    }
    fn to_value(&self) -> Value {
        let value = TupleStructValue::with_capacity(1usize);
        let value = value.with_field(self.as_bytes().to_value());
        value.into()
    }
    fn clone_reflect(&self) -> Box<dyn Reflect> {
        Box::new(self.clone())
    }
    fn debug(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            f.write_fmt(format_args!("{0:#?}", self))
        } else {
            f.write_fmt(format_args!("{0:?}", self))
        }
    }
    fn reflect_owned(self: Box<Self>) -> ReflectOwned {
        ReflectOwned::TupleStruct(self)
    }
    fn reflect_ref(&self) -> ReflectRef<'_> {
        ReflectRef::TupleStruct(self)
    }
    fn reflect_mut(&mut self) -> ReflectMut<'_> {
        ReflectMut::TupleStruct(self)
    }
}

impl FromReflect for Uuid {
    fn from_reflect(reflect: &dyn Reflect) -> Option<Self> {
        let tuple_struct = reflect.reflect_ref().as_tuple_struct()?;
        Some(Self::from_bytes({
            let value = tuple_struct.field_at(0)?;
            if let Some(value) = value.downcast_ref::<[u8; 16]>() {
                value.to_owned()
            } else {
                <[u8; 16] as FromReflect>::from_reflect(value)?.to_owned()
            }
        }))
    }
}

impl TupleStruct for Uuid {
    fn field_at(&self, index: usize) -> Option<&dyn Reflect> {
        match index {
            0usize => Some(self.as_bytes().as_reflect()),
            _ => None,
        }
    }
    /// Can't modify its field
    fn field_at_mut(&mut self, index: usize) -> Option<&mut dyn Reflect> {
        None
    }
    fn fields(&self) -> mirror_mirror::tuple_struct::Iter<'_> {
        mirror_mirror::tuple_struct::Iter::new(self)
    }
    fn fields_mut(&mut self) -> ValueIterMut<'_> {
        let iter = [].into_iter();
        Box::new(iter)
    }
    fn fields_len(&self) -> usize {
        1usize
    }
}
impl From<Uuid> for Value {
    fn from(data: Uuid) -> Value {
        data.to_value()
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::{DescribeType, Reflect};
    #[test]
    fn valid_uuid_impls() {
        let uuid = Uuid::parse_str("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8").unwrap();
        let reflect_obj: &dyn Reflect = uuid.as_reflect();
        let uuid_type = <Uuid as DescribeType>::type_descriptor();
    }
}
