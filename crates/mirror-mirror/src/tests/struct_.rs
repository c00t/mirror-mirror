use std::borrow::Cow;

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use crate::enum_::VariantField;
use crate::key_path;
use crate::key_path::GetTypePath;
use crate::struct_::StructValue;
use crate::type_info::GetMeta;
use crate::DescribeType;
use crate::FromReflect;
use crate::GetField;
use crate::Reflect;
use crate::Struct;
use crate::Value;

use fixed_type_id::{fixed_type_id, FixedId, FixedTypeId, FixedVersion};

fixed_type_id! {
    tests::struct_::Foo;
}

#[derive(Reflect, Default, Clone, Eq, PartialEq, Debug)]
#[reflect(crate_name(crate))]
struct Foo {
    field: i32,
}

#[test]
fn accessing_fields() {
    let foo = Foo { field: 42 };
    let struct_ = foo.reflect_ref().as_struct().unwrap();

    let value = struct_
        .field("field")
        .unwrap()
        .downcast_ref::<i32>()
        .unwrap();

    assert_eq!(*value, 42);

    let value: Value = struct_.to_value();
    assert_eq!(value.get_field::<i32>("field").unwrap(), &42);
    assert_eq!(value.get_field::<i32>("field".to_owned()).unwrap(), &42);
}

#[test]
fn patching() {
    let mut foo = Foo { field: 42 };

    let patch = StructValue::default().with_field("field", 1337);

    foo.patch(&patch);

    assert_eq!(foo.field, 1337);
}

#[test]
fn patching_struct_value() {
    let mut value = StructValue::default().with_field("field", 42);
    let patch = StructValue::default().with_field("field", 1337);
    value.patch(&patch);

    assert_eq!(
        value.field("field").unwrap().downcast_ref::<i32>().unwrap(),
        &1337
    );
}

#[test]
fn from_reflect() {
    let foo = Foo::default();
    let foo_reflect: &dyn Reflect = &foo;

    let foo = <Foo as FromReflect>::from_reflect(foo_reflect).unwrap();

    assert_eq!(foo.field, 0);
}

#[test]
fn fields() {
    let foo = Foo::default();

    for (name, value) in foo.fields() {
        if name == "field" {
            assert_eq!(foo.field, i32::from_reflect(value).unwrap());
        } else {
            panic!("Unknown field {name:?}");
        }
    }
}

#[test]
fn struct_value_from_reflect() {
    let value = StructValue::default().with_field("foo", 42);
    let reflect = value.as_reflect();

    let value = StructValue::from_reflect(reflect).unwrap();

    assert_eq!(
        value.field("foo").unwrap().downcast_ref::<i32>().unwrap(),
        &42,
    );
}

#[test]
fn box_dyn_reflect_as_reflect() {
    let foo = Foo::default();
    let mut box_dyn_reflect = Box::new(foo) as Box<dyn Reflect>;

    assert_eq!(
        box_dyn_reflect
            .reflect_ref()
            .as_struct()
            .unwrap()
            .field("field")
            .unwrap()
            .downcast_ref::<i32>()
            .unwrap(),
        &0,
    );

    box_dyn_reflect.patch(&StructValue::default().with_field("field", 42));

    assert_eq!(
        box_dyn_reflect
            .reflect_ref()
            .as_struct()
            .unwrap()
            .field("field")
            .unwrap()
            .downcast_ref::<i32>()
            .unwrap(),
        &42,
    );

    let foo = <Foo as FromReflect>::from_reflect(box_dyn_reflect.as_reflect()).unwrap();
    assert_eq!(foo, Foo { field: 42 });
}

#[test]
fn deeply_nested() {
    fixed_type_id! {
        tests::struct_::deeply_nested::Foo;
        tests::struct_::deeply_nested::Bar;
        tests::struct_::deeply_nested::Baz;
    }

    #[derive(Reflect, Clone, Debug, Default)]
    #[reflect(crate_name(crate))]
    struct Foo {
        bar: Bar,
    }

    #[derive(Reflect, Clone, Debug, Default)]
    #[reflect(crate_name(crate))]
    struct Bar {
        baz: Baz,
    }

    #[derive(Reflect, Clone, Debug, Default)]
    #[reflect(crate_name(crate))]
    struct Baz {
        qux: i32,
    }

    let foo = Foo {
        bar: Bar {
            baz: Baz { qux: 42 },
        },
    };

    let &forty_two = (|| {
        foo.get_field::<Bar>("bar")?
            .get_field::<Baz>("baz")?
            .get_field::<i32>("qux")
    })()
    .unwrap();

    assert_eq!(forty_two, 42);
}

#[test]
fn from_reflect_with_value() {
    fixed_type_id! {
        tests::struct_::from_reflect_with_value::Foo;
        tests::struct_::from_reflect_with_value::Number;
    }

    #[derive(Debug, Clone, Reflect, Default)]
    #[reflect(crate_name(crate))]
    pub struct Foo {
        pub number: Number,
    }

    #[derive(Debug, Clone, Reflect, Default)]
    #[reflect(crate_name(crate))]
    pub enum Number {
        #[default]
        One,
        Two,
        Three,
    }

    let value = StructValue::new().with_field("number", Number::One);

    assert!(Foo::from_reflect(&value).is_some());
}

#[test]
fn accessing_docs_in_type_info() {
    fixed_type_id! {
        tests::struct_::accessing_docs_in_type_info::Foo;
        tests::struct_::accessing_docs_in_type_info::Inner;
    }

    /// Here are the docs.
    ///
    /// Foo bar.
    #[derive(Reflect, Clone, Debug, Default)]
    #[reflect(crate_name(crate))]
    struct Foo {
        inner: Vec<BTreeMap<String, Vec<Option<Inner>>>>,
    }

    #[derive(Reflect, Clone, Debug)]
    #[reflect(crate_name(crate), opt_out(Default))]
    enum Inner {
        Variant {
            /// Bingo!
            field: String,
        },
    }

    let type_info = <Foo as DescribeType>::type_descriptor();

    assert_eq!(
        type_info.get_type().docs(),
        &[" Here are the docs.", "", " Foo bar."]
    );

    let variant_info = type_info
        .get_type()
        .type_at(&key_path!(.inner[0]["map_key"][0]::Some.0::Variant))
        .unwrap()
        .as_variant()
        .unwrap();
    let field = variant_info.field_types().next().unwrap();
    assert_eq!(field.name().unwrap(), "field");
    assert_eq!(field.docs(), &[" Bingo!"]);
}

// whether we iterate over the fields in a value or the fields in a type we should get the same
// order
#[test]
fn consistent_iteration_order_of_struct_fields() {
    fixed_type_id! {
        tests::struct_::consistent_iteration_order_of_struct_fields::Outer;
        tests::struct_::consistent_iteration_order_of_struct_fields::Inner;
    }

    #[derive(Reflect, Debug, Clone, Default)]
    #[reflect(crate_name(crate))]
    struct Outer {
        inner: Inner,
    }

    #[derive(Reflect, Debug, Clone, Copy, Default)]
    #[reflect(crate_name(crate))]
    struct Inner {
        // the order the fields are declared in is important!
        b: u32,
        a: u32,
    }

    let outer = Outer {
        inner: Inner { a: 1, b: 2 },
    };

    let value = outer.as_reflect().as_struct().unwrap();
    let mut by_value = Vec::new();
    for (outer_field_name, outer_field_value) in value.fields() {
        by_value.push(outer_field_name);
        for (inner_field_name, _) in outer_field_value.as_struct().unwrap().fields() {
            by_value.push(inner_field_name);
        }
    }

    let ty = <Outer as DescribeType>::type_descriptor();
    let ty = ty.as_struct().unwrap();
    let mut by_type = Vec::new();
    for outer_field_ty in ty.field_types() {
        by_type.push(outer_field_ty.name());
        for inner_field_ty in outer_field_ty.get_type().as_struct().unwrap().field_types() {
            by_type.push(inner_field_ty.name());
        }
    }

    assert_eq!(by_value, by_type);
}

#[test]
fn consistent_iteration_order_of_struct_variant_fields() {
    fixed_type_id! {
        tests::struct_::consistent_iteration_order_of_struct_variant_fields::Outer;
        tests::struct_::consistent_iteration_order_of_struct_variant_fields::Inner;
    }

    #[derive(Reflect, Debug, Clone)]
    #[reflect(crate_name(crate), opt_out(Default))]
    struct Outer {
        inner: Inner,
    }

    #[derive(Reflect, Debug, Clone, Copy)]
    #[reflect(crate_name(crate), opt_out(Default))]
    enum Inner {
        A {
            // the order the fields are declared in is important!
            b: u32,
            a: u32,
        },
    }

    let outer = Outer {
        inner: Inner::A { a: 1, b: 2 },
    };

    let value = outer.as_reflect().as_struct().unwrap();
    let mut by_value = Vec::new();
    for (outer_field_name, outer_field_value) in value.fields() {
        by_value.push(outer_field_name);
        for inner_field in outer_field_value.as_enum().unwrap().fields() {
            match inner_field {
                VariantField::Struct(inner_field_name, _) => {
                    by_value.push(inner_field_name);
                }
                VariantField::Tuple(_) => unreachable!(),
            }
        }
    }

    let ty = <Outer as DescribeType>::type_descriptor();
    let ty = ty.as_struct().unwrap();
    let mut by_type = Vec::new();
    for outer_field_ty in ty.field_types() {
        by_type.push(outer_field_ty.name());
        for inner_field_ty in outer_field_ty
            .get_type()
            .as_enum()
            .unwrap()
            .variant("A")
            .unwrap()
            .field_types()
        {
            by_type.push(inner_field_ty.name().unwrap());
        }
    }

    assert_eq!(by_value, by_type);
}

#[test]
fn deserialize_old_struct() {
    mod v1 {
        use fixed_type_id::{fixed_type_id, FixedId, FixedTypeId, FixedVersion};

        fixed_type_id! {
            tests::struct_::deserialize_old_struct::v1::Foo;
        }

        #[derive(mirror_mirror_1::Reflect, Debug, Clone)]
        #[reflect(crate_name(mirror_mirror_1))]
        pub struct Foo {
            pub n: i32,
        }
    }

    mod v2 {
        use fixed_type_id::{fixed_type_id, FixedId, FixedTypeId, FixedVersion};

        fixed_type_id! {
            tests::struct_::deserialize_old_struct::v2::Foo;
        }

        #[derive(crate::Reflect, Default, Debug, Clone)]
        #[reflect(crate_name(crate))]
        pub struct Foo {
            pub n: i32,
        }
    }

    // deserializing value
    let n = 123;
    let v1_foo = mirror_mirror_1::Reflect::to_value(&v1::Foo { n });
    let v1_ron = ron::to_string(&v1_foo).unwrap();

    let v2_value = ron::from_str::<crate::Value>(&v1_ron).unwrap();
    let v2_foo = <v2::Foo as crate::FromReflect>::from_reflect(&v2_value).unwrap();

    assert_eq!(n, v2_foo.n);

    // deserializing type descriptor
    let v1_type_descriptor = <v1::Foo as mirror_mirror_1::DescribeType>::type_descriptor();
    let v1_ron = ron::to_string(&v1_type_descriptor).unwrap();

    let v2_type_descriptor =
        ron::from_str::<Cow<'static, crate::type_info::TypeDescriptor>>(&v1_ron).unwrap();

    let name_type = v2_type_descriptor
        .as_struct()
        .unwrap()
        .field_type("n")
        .unwrap()
        .get_type()
        .as_scalar()
        .unwrap();
    assert!(matches!(name_type, crate::type_info::ScalarType::i32));
}

#[test]
fn default_value() {
    fixed_type_id! {
        tests::struct_::default_value::Foo;
        tests::struct_::default_value::Bar;
    }

    #[derive(Reflect, Debug, Clone, Copy)]
    #[reflect(crate_name(crate))]
    struct Foo {
        a: u32,
        b: u32,
    }

    impl Default for Foo {
        fn default() -> Self {
            Foo { a: 1, b: 2 }
        }
    }

    #[derive(Reflect, Debug, Clone, Copy)]
    #[reflect(crate_name(crate), opt_out(Default))]
    struct Bar {
        a: u32,
        b: u32,
    }

    let foo_descriptor = <Foo as DescribeType>::type_descriptor();
    let bar_descriptor = <Bar as DescribeType>::type_descriptor();

    assert!(foo_descriptor.has_default_value());
    assert!(!bar_descriptor.has_default_value());

    let foo_default = Foo::default().to_value();

    assert_eq!(foo_descriptor.default_value(), Some(foo_default));
    assert_eq!(bar_descriptor.default_value(), None);
}

#[test]
fn field_named_named() {
    fixed_type_id! {
        tests::struct_::field_named_named::A;
    }

    #[derive(Reflect, Debug, Clone)]
    #[reflect(crate_name(crate), opt_out(Default))]
    struct A {
        name: String,
        value: (),
        reflect: (),
        enum_: (),
        struct_: (),
    }

    let mut a = A {
        name: "foo".to_owned(),
        value: (),
        reflect: (),
        enum_: (),
        struct_: (),
    };

    assert_eq!(
        a.as_reflect()
            .as_struct()
            .unwrap()
            .field("name")
            .unwrap()
            .downcast_ref::<String>()
            .unwrap(),
        "foo"
    );
    assert_eq!(
        a.as_reflect_mut()
            .as_struct_mut()
            .unwrap()
            .field_mut("name")
            .unwrap()
            .downcast_mut::<String>()
            .unwrap(),
        "foo"
    );
}
