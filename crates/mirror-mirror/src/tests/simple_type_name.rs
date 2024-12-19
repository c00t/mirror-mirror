use alloc::collections::BTreeMap;

use crate::type_info::SimpleTypeName;

use fixed_type_id::{
    fixed_type_id, fstr_to_str, type_name, usize_to_str, ConstTypeName, FixedId, FixedTypeId,
    FixedVersion,
};

fn simple_type_name<T: FixedTypeId>() -> String {
    SimpleTypeName::new_from_type::<T>().to_string()
}

#[test]
fn works() {
    impl<'a, const N: usize> FixedTypeId for Foo<'a, N> {
        const TYPE_NAME: &'static str = fstr_to_str(&Self::TYPE_NAME_FSTR);
    }

    impl<'a, const N: usize> ConstTypeName for Foo<'a, N> {
        const RAW_SLICE: &'static [&'static str] =
            &["tests::simple_type_name::works::Foo<", usize_to_str(N), ">"];
    }

    #[allow(dead_code)]
    #[allow(dead_code)]
    struct Foo<'a, const N: usize>(&'a ());

    assert_eq!(simple_type_name::<String>(), "String");
    assert_eq!(simple_type_name::<i32>(), "i32");
    assert_eq!(simple_type_name::<bool>(), "bool");
    assert_eq!(simple_type_name::<()>(), "()");
    assert_eq!(simple_type_name::<(i32,)>(), "(i32,)");
    assert_eq!(simple_type_name::<(i32, String)>(), "(i32, String)");
    assert_eq!(simple_type_name::<Vec<i32>>(), "Vec<i32>");
    assert_eq!(simple_type_name::<Vec<&()>>(), "Vec<&()>");
    assert_eq!(simple_type_name::<Vec<&mut ()>>(), "Vec<&mut ()>");
    assert_eq!(simple_type_name::<Vec<&'static ()>>(), "Vec<&()>");
    assert_eq!(simple_type_name::<Vec<&'static mut ()>>(), "Vec<&mut ()>");
    assert_eq!(simple_type_name::<Option<String>>(), "Option<String>");
    assert_eq!(
        simple_type_name::<BTreeMap<i32, String>>(),
        "BTreeMap<i32, String>"
    );
    assert_eq!(
        simple_type_name::<BTreeMap<Vec<(i32, Option<bool>)>, String>>(),
        "BTreeMap<Vec<(i32, Option<bool>)>, String>"
    );
    assert_eq!(simple_type_name::<[i32; 10]>(), "[i32; 10]");
    assert_eq!(
        simple_type_name::<[BTreeMap<i32, i32>; 10]>(),
        "[BTreeMap<i32, i32>; 10]"
    );
    // type names don't include lifetimes
    assert_eq!(simple_type_name::<Foo<'static, 10>>(), "Foo<10>");
    assert_eq!(simple_type_name::<Box<dyn std::any::Any>>(), "Box<dyn Any>");
}

#[test]
fn type_inside_unnamed_const() {
    trait A {
        type T;
    }

    struct Foo;

    const _: () = {
        struct Bar<T>(T);

        fixed_type_id! {
            tests::simple_type_name::type_inside_unnamed_const::Bar<std::string::String>;
        }

        impl A for Foo {
            type T = Bar<String>;
        }
    };

    assert_eq!(simple_type_name::<<Foo as A>::T>(), "Bar<String>");
}
