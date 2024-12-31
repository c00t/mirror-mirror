use alloc::vec::Vec;

use crate::tuple_struct::TupleStructValue;
use crate::DescribeType;
use crate::FromReflect;
use crate::GetField;
use crate::Reflect;
use crate::TupleStruct;

use fixed_type_id::{fixed_type_id, type_name, type_id, FixedId, FixedTypeId, FixedVersion};

#[test]
fn tuple_value() {
    let mut tuple = TupleStructValue::new().with_field(1_i32).with_field(false);

    assert_eq!(tuple.get_field::<i32>(0).unwrap(), &1);
    assert_eq!(tuple.get_field::<bool>(1).unwrap(), &false);

    tuple.patch(&TupleStructValue::new().with_field(42_i32));
    assert_eq!(tuple.get_field::<i32>(0).unwrap(), &42);
    assert_eq!(tuple.get_field::<bool>(1).unwrap(), &false);
}

#[test]
fn static_tuple() {
    fixed_type_id! {
        tests::tuple_struct::static_tuple::A;
    }

    #[derive(Reflect, Default, Clone, Eq, PartialEq, Debug)]
    #[reflect(crate_name(crate))]
    struct A(i32, bool);

    let mut tuple = A(1_i32, false);

    assert_eq!(tuple.get_field::<i32>(0).unwrap(), &1);
    assert_eq!(tuple.get_field::<bool>(1).unwrap(), &false);

    tuple.patch(&TupleStructValue::new().with_field(42_i32));
    assert_eq!(tuple.get_field::<i32>(0).unwrap(), &42);
    assert_eq!(tuple.get_field::<bool>(1).unwrap(), &false);

    let mut tuple = A::from_reflect(&tuple.to_value()).unwrap();
    assert!(matches!(tuple, A(42, false)));

    let fields = tuple.fields().collect::<Vec<_>>();
    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].downcast_ref::<i32>().unwrap(), &42);
    assert_eq!(fields[1].downcast_ref::<bool>().unwrap(), &false);

    tuple.field_at_mut(1).unwrap().patch(&true);
    assert!(tuple.1);
}

#[test]
fn from_reflect_with_value() {
    fixed_type_id! {
        tests::tuple_struct::from_reflect_with_value::Foo;
        tests::tuple_struct::from_reflect_with_value::Number;
    }

    #[derive(Debug, Clone, Reflect, Default)]
    #[reflect(crate_name(crate))]
    pub struct Foo(Number);

    #[derive(Debug, Clone, Reflect, Default)]
    #[reflect(crate_name(crate))]
    pub enum Number {
        #[default]
        One,
        Two,
        Three,
    }

    let value = TupleStructValue::new().with_field(Number::One);

    assert!(Foo::from_reflect(&value).is_some());
}
#[test]
fn default_value() {
    fixed_type_id! {
        tests::tuple_struct::default_value::Foo;
        tests::tuple_struct::default_value::Bar;
    }

    #[derive(Reflect, Debug, Clone, Copy)]
    #[reflect(crate_name(crate))]
    struct Foo(u32, u32);

    impl Default for Foo {
        fn default() -> Self {
            Foo(1, 2)
        }
    }

    #[derive(Reflect, Debug, Clone, Copy)]
    #[reflect(crate_name(crate), opt_out(Default))]
    struct Bar(u32, u32);

    let foo_descriptor = <Foo as DescribeType>::type_descriptor();
    let bar_descriptor = <Bar as DescribeType>::type_descriptor();

    assert!(foo_descriptor.has_default_value());
    assert!(!bar_descriptor.has_default_value());

    let foo_default = Foo::default().to_value();

    assert_eq!(foo_descriptor.default_value(), Some(foo_default));
    assert_eq!(bar_descriptor.default_value(), None);
}
