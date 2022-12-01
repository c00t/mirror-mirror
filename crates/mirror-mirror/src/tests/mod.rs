use crate as mirror_mirror;
use crate::Reflect;

mod enum_;
mod key_path;
mod list;
mod map;
mod meta;
mod struct_;
mod tuple;
mod tuple_struct;

#[derive(Reflect)]
#[reflect(opt_out(Debug, Clone))]
#[allow(dead_code)]
struct DebugOptOut;

#[allow(warnings)]
fn box_t_is_reflectable<T>(t: Box<T>)
where
    T: Reflect,
{
    let _ = t.as_reflect();
}

mod complex_types {
    #![allow(dead_code)]

    use crate as mirror_mirror;
    use crate::Reflect;
    use std::collections::BTreeMap;

    #[derive(Reflect, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
    struct A {
        a: String,
        b: Vec<B>,
        d: BTreeMap<B, Vec<A>>,
    }

    #[derive(Reflect, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
    enum B {
        C(C),
        D { d: D },
    }

    #[derive(Reflect, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
    struct C(String, i32, Vec<bool>);

    #[derive(Reflect, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
    struct D;
}

mod skip {
    #![allow(dead_code)]

    use super::*;

    #[derive(Reflect, Debug, Clone)]
    struct TestStruct {
        #[reflect(skip)]
        not_reflect: NotReflect,
    }

    #[derive(Reflect, Debug, Clone)]
    struct TestTupleStruct(#[reflect(skip)] NotReflect);

    // TODO(david): support #[reflection(skip)] on fields inside variants
    #[derive(Reflect, Debug, Clone)]
    #[allow(clippy::enum_variant_names)]
    enum TestEnum {
        #[reflect(skip)]
        SkipStructVariant { not_reflect: NotReflect },
        // SkipStructField {
        //     #[reflect(skip)]
        //     not_reflect: NotReflect,
        // },
        #[reflect(skip)]
        SkipTupleVariant(NotReflect),
        // SkipTupleField(#[reflect(skip)] NotReflect),
        #[reflect(skip)]
        SkipUnitVariant,
    }

    #[derive(Debug, Clone, Default)]
    struct NotReflect;
}
