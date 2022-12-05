use crate::array::Array;
use crate::iter::ValueIter;
use crate::iter::ValueIterMut;
use crate::type_info::graph::ArrayInfoNode;
use crate::type_info::graph::Id;
use crate::type_info::graph::TypeInfoGraph;
use crate::FromReflect;
use crate::Reflect;
use crate::ReflectMut;
use crate::ReflectRef;
use crate::TypeInfoRoot;
use crate::Typed;
use crate::Value;
use std::any::Any;
use std::fmt;

impl<T, const N: usize> Reflect for [T; N]
where
    T: FromReflect + Typed,
{
    fn type_info(&self) -> TypeInfoRoot {
        impl<T, const N: usize> Typed for [T; N]
        where
            T: Typed,
        {
            fn build(graph: &mut TypeInfoGraph) -> Id {
                graph.get_or_build_with::<Self, _>(|graph| ArrayInfoNode::new::<Self, T, N>(graph))
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

    fn reflect_ref(&self) -> ReflectRef<'_> {
        ReflectRef::Array(self)
    }

    fn reflect_mut(&mut self) -> ReflectMut<'_> {
        ReflectMut::Array(self)
    }

    fn patch(&mut self, value: &dyn Reflect) {
        if let Some(array) = value.reflect_ref().as_array() {
            for (idx, new_value) in array.iter().enumerate() {
                if let Some(value) = self.get_mut(idx) {
                    value.patch(new_value);
                }
            }
        }
    }

    fn to_value(&self) -> Value {
        let data = self.iter().map(Reflect::to_value).collect();
        Value::List(data)
    }

    fn clone_reflect(&self) -> Box<dyn Reflect> {
        let value = self.to_value();
        Box::new(Self::from_reflect(&value).unwrap())
    }

    fn debug(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T, const N: usize> Array for [T; N]
where
    T: FromReflect + Typed,
{
    fn get(&self, index: usize) -> Option<&dyn Reflect> {
        self.as_slice().get(index).map(|value| value.as_reflect())
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut dyn Reflect> {
        self.as_mut_slice()
            .get_mut(index)
            .map(|value| value.as_reflect_mut())
    }

    fn len(&self) -> usize {
        N
    }

    fn is_empty(&self) -> bool {
        N == 0
    }

    fn iter(&self) -> ValueIter<'_> {
        let iter = self.as_slice().iter().map(|value| value.as_reflect());
        ValueIter::new(iter)
    }

    fn iter_mut(&mut self) -> ValueIterMut<'_> {
        let iter = self
            .as_mut_slice()
            .iter_mut()
            .map(|value| value.as_reflect_mut());
        ValueIterMut::new(iter)
    }
}

impl<T, const N: usize> FromReflect for [T; N]
where
    T: FromReflect + Typed,
{
    fn from_reflect(reflect: &dyn Reflect) -> Option<Self> {
        Vec::<T>::from_reflect(reflect)?.try_into().ok()
    }
}

impl<T, const N: usize> From<[T; N]> for Value
where
    T: Reflect,
{
    fn from(list: [T; N]) -> Self {
        let list = list
            .iter()
            .map(|value| value.to_value())
            .collect::<Vec<_>>();
        Value::List(list)
    }
}