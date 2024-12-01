use crate::Reflect;

mod array;
mod enum_;
mod key_path;
mod list;
mod map;
mod meta;
mod simple_type_name;
mod struct_;
mod tuple;
mod tuple_struct;
mod type_info;
mod value;

use fixed_type_id::{fixed_type_id, FixedId, FixedTypeId, FixedVersion};

fixed_type_id! {
    tests::DebugOptOut;
    tests::ContainsBoxed;
}

#[derive(Reflect)]
#[reflect(crate_name(crate), opt_out(Debug, Clone))]
#[allow(dead_code)]
struct DebugOptOut;

#[derive(Reflect)]
#[reflect(crate_name(crate), opt_out(Debug, Clone))]
#[allow(dead_code)]
struct ContainsBoxed(Box<f32>);

mod complex_types {
    #![allow(dead_code)]

    use alloc::collections::BTreeMap;

    use crate::Reflect;

    use fixed_type_id::{fixed_type_id, FixedId, FixedTypeId, FixedVersion};

    fixed_type_id! {
        tests::complex_types::A;
        tests::complex_types::B;
        tests::complex_types::C;
        tests::complex_types::D;
    }

    #[derive(Reflect, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
    #[reflect(crate_name(crate))]
    struct A {
        a: String,
        b: Vec<B>,
        d: BTreeMap<B, Vec<A>>,
    }

    #[derive(Reflect, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
    #[reflect(crate_name(crate))]
    enum B {
        C(C),
        D { d: D },
    }

    #[derive(Reflect, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
    #[reflect(crate_name(crate))]
    struct C(String, i32, Vec<bool>);

    #[derive(Reflect, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
    #[reflect(crate_name(crate))]
    struct D;
}

mod skip {
    #![allow(dead_code)]

    use super::*;

    use fixed_type_id::{fixed_type_id, FixedId, FixedTypeId, FixedVersion};

    fixed_type_id! {
        tests::skip::TestStruct;
        tests::skip::TestTupleStruct;
        tests::skip::TestEnum;
        tests::skip::NotReflect;
    }

    #[derive(Reflect, Debug, Clone)]
    #[reflect(crate_name(crate))]
    struct TestStruct {
        #[reflect(skip)]
        not_reflect: NotReflect,
    }

    #[derive(Reflect, Debug, Clone)]
    #[reflect(crate_name(crate))]
    struct TestTupleStruct(#[reflect(skip)] NotReflect);

    #[derive(Reflect, Debug, Clone)]
    #[reflect(crate_name(crate))]
    #[allow(clippy::enum_variant_names)]
    enum TestEnum {
        #[reflect(skip)]
        SkipStructVariant {
            not_reflect: NotReflect,
        },
        SkipStructField {
            #[reflect(skip)]
            not_reflect: NotReflect,
        },
        #[reflect(skip)]
        SkipTupleVariant(NotReflect),
        SkipTupleField(#[reflect(skip)] NotReflect),
        #[reflect(skip)]
        SkipUnitVariant,
    }

    #[derive(Debug, Clone, Default)]
    struct NotReflect;
}

mod option_f32 {
    #![allow(dead_code)]

    use super::*;

    use fixed_type_id::{fixed_type_id, FixedId, FixedTypeId, FixedVersion};

    fixed_type_id! {
        tests::option_f32::Foo;
    }

    #[derive(Debug, Clone, Reflect)]
    #[reflect(crate_name(crate))]
    struct Foo {
        maybe_float: Option<f32>,
        maybe_string: Option<String>,
    }
}

mod derive_foreign {
    #![allow(dead_code)]

    use mirror_mirror_macros::*;

    use crate::DescribeType;
    use crate::FromReflect;

    use fixed_type_id::{
        fixed_type_id, fstr_to_str, implement_wrapper_fixed_type_id, ConstTypeName, FixedId,
        FixedTypeId, FixedVersion,
    };

    enum Foo<A, B>
    where
        A: FromReflect + DescribeType,
        B: FromReflect + DescribeType,
    {
        Struct { a: A },
        Tuple(B),
        Unit,
    }

    // too complex, implement it manually
    impl<A, B> FixedTypeId for Foo<A, B>
    where
        A: FromReflect + DescribeType,
        B: FromReflect + DescribeType,
    {
        const TYPE_NAME: &'static str = fstr_to_str(&Self::TYPE_NAME_FSTR);
    }

    impl<A, B> ConstTypeName for Foo<A, B>
    where
        A: FromReflect + DescribeType,
        B: FromReflect + DescribeType,
    {
        const RAW_SLICE: &'static [&'static str] = &[
            "tests::derive_foreign::Foo<",
            A::TYPE_NAME,
            ", ",
            B::TYPE_NAME,
            ">",
        ];
    }

    __private_derive_reflect_foreign! {
        #[reflect(opt_out(Clone, Debug), crate_name(crate))]
        enum Foo<A, B>
        where
            A: FromReflect + DescribeType,
            B: FromReflect + DescribeType,
        {
            Struct { a: A },
            Tuple(B),
            Unit,
        }
    }

    struct Bar<A, B>
    where
        A: FromReflect + DescribeType,
        B: FromReflect + DescribeType,
    {
        a: A,
        b: B,
    }

    // too complex, implement it manually
    impl<A, B> FixedTypeId for Bar<A, B>
    where
        A: FromReflect + DescribeType,
        B: FromReflect + DescribeType,
    {
        const TYPE_NAME: &'static str = fstr_to_str(&Self::TYPE_NAME_FSTR);
    }

    impl<A, B> ConstTypeName for Bar<A, B>
    where
        A: FromReflect + DescribeType,
        B: FromReflect + DescribeType,
    {
        const RAW_SLICE: &'static [&'static str] = &[
            "tests::derive_foreign::Bar<",
            A::TYPE_NAME,
            ", ",
            B::TYPE_NAME,
            ">",
        ];
    }

    __private_derive_reflect_foreign! {
        #[reflect(opt_out(Clone, Debug), crate_name(crate))]
        struct Bar<A, B>
        where
            A: FromReflect + DescribeType,
            B: FromReflect + DescribeType,
        {
            a: A,
            b: B,
        }
    }

    struct Baz<A, B>(A, B)
    where
        A: FromReflect + DescribeType,
        B: FromReflect + DescribeType;

    // too complex, implement it manually
    impl<A, B> FixedTypeId for Baz<A, B>
    where
        A: FromReflect + DescribeType,
        B: FromReflect + DescribeType,
    {
        const TYPE_NAME: &'static str = fstr_to_str(&Self::TYPE_NAME_FSTR);
    }

    impl<A, B> ConstTypeName for Baz<A, B>
    where
        A: FromReflect + DescribeType,
        B: FromReflect + DescribeType,
    {
        const RAW_SLICE: &'static [&'static str] = &[
            "tests::derive_foreign::Baz<",
            A::TYPE_NAME,
            ", ",
            B::TYPE_NAME,
            ">",
        ];
    }

    __private_derive_reflect_foreign! {
        #[reflect(opt_out(Clone, Debug), crate_name(crate))]
        struct Baz<A, B>(A, B)
        where
            A: FromReflect + DescribeType,
            B: FromReflect + DescribeType;
    }

    fixed_type_id! {
        tests::derive_foreign::Qux;
    }

    struct Qux;

    __private_derive_reflect_foreign! {
        #[reflect(opt_out(Clone, Debug), crate_name(crate))]
        struct Qux;
    }
}

mod from_reflect_opt_out {
    #![allow(warnings)]

    use super::*;
    use crate::FromReflect;

    use fixed_type_id::{fixed_type_id, FixedId, FixedTypeId, FixedVersion};

    fixed_type_id! {
        tests::from_reflect_opt_out::Percentage;
    }

    #[derive(Reflect, Debug, Clone, Copy, PartialEq)]
    #[reflect(crate_name(crate), opt_out(FromReflect))]
    struct Percentage(f32);

    impl FromReflect for Percentage {
        fn from_reflect(reflect: &dyn Reflect) -> Option<Self> {
            if let Some(this) = reflect.downcast_ref::<Self>() {
                Some(*this)
            } else if let Some(value) = f32::from_reflect(reflect) {
                Some(Self(value.clamp(0.0, 100.0)))
            } else if let Some(value) = f64::from_reflect(reflect) {
                Some(Self((value as f32).clamp(0.0, 100.0)))
            } else {
                None
            }
        }
    }

    #[test]
    fn works() {
        assert_eq!(
            Percentage::from_reflect(&Percentage(10.0)).unwrap(),
            Percentage(10.0)
        );

        assert_eq!(Percentage::from_reflect(&10.0).unwrap(), Percentage(10.0));

        assert_eq!(
            Percentage::from_reflect(&1337.0).unwrap(),
            Percentage(100.0)
        );
    }

    fixed_type_id! {
        tests::from_reflect_opt_out::B;
        tests::from_reflect_opt_out::C;
    }

    #[derive(Reflect, Debug, Clone)]
    #[reflect(crate_name(crate), opt_out(FromReflect))]
    struct B {
        n: f32,
    }

    impl FromReflect for B {
        fn from_reflect(reflect: &dyn Reflect) -> Option<Self> {
            None
        }
    }

    #[derive(Reflect, Debug, Clone)]
    #[reflect(crate_name(crate), opt_out(FromReflect))]
    enum C {
        A(f32),
    }

    impl FromReflect for C {
        fn from_reflect(reflect: &dyn Reflect) -> Option<Self> {
            None
        }
    }
}

mod from_reflect_with {
    #![allow(warnings)]

    use super::*;
    use crate::FromReflect;

    use fixed_type_id::{fixed_type_id, FixedId, FixedTypeId, FixedVersion};

    fixed_type_id! {
        tests::from_reflect_with::A;
        tests::from_reflect_with::B;
        tests::from_reflect_with::C;
    }

    #[derive(Reflect, Debug, Clone, Copy, PartialEq)]
    #[reflect(crate_name(crate))]
    struct A {
        #[reflect(from_reflect_with(clamp_ratio))]
        a: f32,
    }

    #[derive(Reflect, Debug, Clone, Copy, PartialEq)]
    #[reflect(crate_name(crate))]
    struct B(#[reflect(from_reflect_with(clamp_ratio))] f32);

    #[derive(Reflect, Debug, Clone, Copy, PartialEq)]
    #[reflect(crate_name(crate))]
    enum C {
        C(#[reflect(from_reflect_with(clamp_ratio))] f32),
        D {
            #[reflect(from_reflect_with(clamp_ratio))]
            d: f32,
        },
    }

    fn clamp_ratio(ratio: &dyn Reflect) -> Option<f32> {
        Some(ratio.downcast_ref::<f32>()?.clamp(0.0, 1.0))
    }

    #[test]
    fn works() {
        assert_eq!(A::from_reflect(&A { a: 100.0 }).unwrap(), A { a: 1.0 });
        assert_eq!(A::from_reflect(&A { a: -100.0 }).unwrap(), A { a: 0.0 });

        assert_eq!(B::from_reflect(&B(100.0)).unwrap(), B(1.0));
        assert_eq!(B::from_reflect(&B(-100.0)).unwrap(), B(0.0));

        assert_eq!(C::from_reflect(&C::C(100.0)).unwrap(), C::C(1.0));
        assert_eq!(C::from_reflect(&C::C(-100.0)).unwrap(), C::C(0.0));

        assert_eq!(
            C::from_reflect(&C::D { d: 100.0 }).unwrap(),
            C::D { d: 1.0 }
        );
        assert_eq!(
            C::from_reflect(&C::D { d: -100.0 }).unwrap(),
            C::D { d: 0.0 }
        );
    }
}
