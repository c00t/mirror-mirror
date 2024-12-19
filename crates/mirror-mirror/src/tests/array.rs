use crate::DescribeType;
use crate::FromReflect;
use crate::Reflect;
use crate::ReflectMut;
use crate::ReflectOwned;
use crate::ReflectRef;
use fixed_type_id::{fixed_type_id, type_name, FixedId, FixedTypeId, FixedVersion};

#[test]
fn from_default_tuple() {
    fixed_type_id! {
        tests::array::Foo;
    }

    #[derive(Debug, Clone, Default, Reflect, PartialEq)]
    #[reflect(crate_name(crate))]
    struct Foo([i32; 5]);

    let foo_default_value = <Foo as DescribeType>::type_descriptor()
        .default_value()
        .unwrap();

    let foo = Foo::from_reflect(&foo_default_value).unwrap();

    assert_eq!(foo, Foo([0, 0, 0, 0, 0]))
}

#[test]
fn from_default_named() {
    fixed_type_id! {
        tests::array::from_default_named::Foo;
    }

    #[derive(Debug, Clone, Default, Reflect, PartialEq)]
    #[reflect(crate_name(crate))]
    struct Foo {
        array: [i32; 5],
    }

    let foo_default_value = <Foo as DescribeType>::type_descriptor()
        .default_value()
        .unwrap();

    assert_eq!(
        Foo::from_reflect(&foo_default_value).unwrap(),
        Foo {
            array: [0, 0, 0, 0, 0],
        }
    );
}

#[test]
fn casting_array_to_list() {
    let mut array: [i32; 5] = [0, 0, 0, 0, 0];
    assert!(array.as_list().is_none());
    assert!(array.as_list_mut().is_none());
    assert!(Box::new(array).into_list().is_none());

    // there is no `Value::Array`. Arrays converted to `Value` will become `Value::Array`, which
    // does support `as_array`
}

#[test]
fn casting_array_to_array() {
    let mut array: [i32; 5] = [0, 0, 0, 0, 0];
    assert!(array.as_array().is_some());
    assert!(array.as_array_mut().is_some());
    assert!(Box::new(array).into_array().is_some());

    let mut array: [i32; 5] = [0, 0, 0, 0, 0];
    assert!(matches!(array.reflect_ref(), ReflectRef::Array(_)));
    assert!(matches!(array.reflect_mut(), ReflectMut::Array(_)));
    assert!(matches!(
        Box::new(array).reflect_owned(),
        ReflectOwned::Array(_),
    ));
}
