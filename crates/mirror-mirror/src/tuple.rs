use crate::iter::ValueIter;
use crate::iter::ValueIterMut;
use crate::type_info::graph::Id;
use crate::type_info::graph::TupleInfoNode;
use crate::type_info::graph::TypeInfoGraph;
use crate::type_info::graph::TypeInfoNode;
use crate::type_info::graph::UnnamedFieldNode;
use crate::FromReflect;
use crate::Reflect;
use crate::ReflectMut;
use crate::ReflectRef;
use crate::TypeInfoRoot;
use crate::Typed;
use crate::Value;
use serde::Deserialize;
use serde::Serialize;
use speedy::Readable;
use speedy::Writable;
use std::any::Any;
use std::fmt;
use std::fmt::Debug;

pub trait Tuple: Reflect {
    fn element(&self, index: usize) -> Option<&dyn Reflect>;

    fn element_mut(&mut self, index: usize) -> Option<&mut dyn Reflect>;

    fn elements(&self) -> ValueIter<'_>;

    fn elements_mut(&mut self) -> ValueIterMut<'_>;
}

impl fmt::Debug for dyn Tuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_reflect().debug(f)
    }
}

#[derive(
    Default,
    Readable,
    Writable,
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
)]
pub struct TupleValue {
    elements: Vec<Value>,
}

impl TupleValue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_element(mut self, value: impl Into<Value>) -> Self {
        self.push_element(value);
        self
    }

    pub fn push_element(&mut self, value: impl Into<Value>) {
        self.elements.push(value.into());
    }
}

impl Tuple for TupleValue {
    fn element(&self, index: usize) -> Option<&dyn Reflect> {
        Some(self.elements.get(index)?.as_reflect())
    }

    fn element_mut(&mut self, index: usize) -> Option<&mut dyn Reflect> {
        Some(self.elements.get_mut(index)?.as_reflect_mut())
    }

    fn elements(&self) -> ValueIter<'_> {
        let iter = self.elements.iter().map(|value| value.as_reflect());
        ValueIter::new(iter)
    }

    fn elements_mut(&mut self) -> ValueIterMut<'_> {
        let iter = self.elements.iter_mut().map(|value| value.as_reflect_mut());
        ValueIterMut::new(iter)
    }
}

impl Reflect for TupleValue {
    fn type_info(&self) -> TypeInfoRoot {
        impl Typed for TupleValue {
            fn build(graph: &mut TypeInfoGraph) -> Id {
                graph.get_or_build_with::<Self, _>(|_graph| TypeInfoNode::Tuple(None))
            }
        }
        <Self as Typed>::type_info()
    }

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

    fn patch(&mut self, value: &dyn Reflect) {
        if let Some(tuple) = value.reflect_ref().as_tuple() {
            for (index, value) in self.elements_mut().enumerate() {
                if let Some(new_value) = tuple.element(index) {
                    value.patch(new_value);
                }
            }
        }
    }

    fn to_value(&self) -> Value {
        self.clone().into()
    }

    fn clone_reflect(&self) -> Box<dyn Reflect> {
        Box::new(self.clone())
    }

    fn debug(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:#?}", self)
        } else {
            write!(f, "{:?}", self)
        }
    }

    fn reflect_ref(&self) -> ReflectRef<'_> {
        ReflectRef::Tuple(self)
    }

    fn reflect_mut(&mut self) -> ReflectMut<'_> {
        ReflectMut::Tuple(self)
    }
}

impl FromReflect for TupleValue {
    fn from_reflect(reflect: &dyn Reflect) -> Option<Self> {
        let tuple = reflect.reflect_ref().as_tuple()?;
        let this = tuple
            .elements()
            .fold(TupleValue::default(), |builder, value| {
                builder.with_element(value.to_value())
            });
        Some(this)
    }
}

macro_rules! impl_tuple {
    ($($ident:ident),* $(,)?) => {
        #[allow(non_snake_case, unused_mut, unused_variables)]
        impl<$($ident,)*> Typed for ($($ident,)*)
        where
            $($ident: Reflect + Typed + Clone,)*
        {
            fn build(graph: &mut TypeInfoGraph) -> Id {
                graph.get_or_build_with::<Self, _>(|graph| {
                    let fields = &[
                        $(
                            UnnamedFieldNode::new::<$ident>(Default::default(), graph),
                        )*
                    ];
                    TupleInfoNode::new::<Self>(fields, Default::default())
                })
            }
        }

        #[allow(non_snake_case, unused_mut, unused_variables)]
        impl<$($ident,)*> Reflect for ($($ident,)*)
        where
            $($ident: Reflect + Typed + Clone,)*
        {
            fn type_info(&self) -> TypeInfoRoot {
                <Self as Typed>::type_info()
            }

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

            #[allow(unused_assignments)]
            fn patch(&mut self, value: &dyn Reflect) {
                if let Some(tuple) = value.reflect_ref().as_tuple() {
                    let ($($ident,)*) = self;
                    let mut i = 0;
                    $(
                        if let Some(element) = tuple.element(i) {
                            $ident.patch(element);
                        }
                        i += 1;
                    )*
                }
            }

            fn to_value(&self) -> Value {
                let ($($ident,)*) = self;
                let mut value = TupleValue::new();
                $(
                    value = value.with_element($ident.to_value());
                )*
                value.into()
            }

            fn clone_reflect(&self) -> Box<dyn Reflect> {
                Box::new(self.clone())
            }

            fn debug(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", std::any::type_name::<Self>())
            }

            fn reflect_ref(&self) -> ReflectRef<'_> {
                ReflectRef::Tuple(self)
            }

            fn reflect_mut(&mut self) -> ReflectMut<'_> {
                ReflectMut::Tuple(self)
            }
        }

        #[allow(non_snake_case, unused_mut, unused_assignments, unused_variables)]
        impl<$($ident,)*> Tuple for ($($ident,)*)
        where
            $($ident: Reflect + Typed + Clone,)*
        {
            fn element(&self, index: usize) -> Option<&dyn Reflect> {
                let mut i = 0;
                let ($($ident,)*) = self;
                $(
                    if index == i {
                        return Some($ident);
                    }
                    i += 1;
                )*
                None
            }

            fn element_mut(&mut self, index: usize) -> Option<&mut dyn Reflect> {
                let mut i = 0;
                let ($($ident,)*) = self;
                $(
                    if index == i {
                        return Some($ident);
                    }
                    i += 1;
                )*
                None
            }

            fn elements(&self) -> ValueIter<'_> {
                let ($($ident,)*) = self;
                ValueIter::new([$($ident.as_reflect(),)*])
            }

            fn elements_mut(&mut self) -> ValueIterMut<'_> {
                let ($($ident,)*) = self;
                ValueIterMut::new([$($ident.as_reflect_mut(),)*])
            }
        }

        #[allow(non_snake_case, unused_mut, unused_assignments, unused_variables)]
        impl<$($ident,)*> FromReflect for ($($ident,)*)
        where
            $($ident: FromReflect + Typed + Clone,)*
        {
            fn from_reflect(reflect: &dyn Reflect) -> Option<Self> {
                Some((
                    $($ident::from_reflect(reflect)?,)*
                ))
            }
        }
    };
}

impl_tuple!();
impl_tuple!(T1);
impl_tuple!(T1, T2);
impl_tuple!(T1, T2, T3);
impl_tuple!(T1, T2, T3, T4);
impl_tuple!(T1, T2, T3, T4, T5);
impl_tuple!(T1, T2, T3, T4, T5, T6);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);

impl<V> FromIterator<V> for TupleValue
where
    V: Reflect,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = V>,
    {
        let mut out = Self::default();
        for value in iter {
            out.push_element(value.to_value());
        }
        out
    }
}
