use super::*;

impl<A: Array> Eq for StackVec<A>
where
    A::Item : Eq,
{}

impl<A: Array> PartialEq for StackVec<A>
where
    A::Item : PartialEq,
{
    #[inline(always)]
    fn eq (
        self: &Self,
        other: &Self,
    ) -> bool
    {
        self.as_slice().eq(other.as_slice())
    }
}

impl<A: Array> hash::Hash for StackVec<A>
where
    A::Item : hash::Hash,
{
    fn hash<H: hash::Hasher> (
        self: &Self,
        state: &mut H,
    )
    {
        self.as_slice().hash(state)
    }
}

impl<A: Array> Clone for StackVec<A>
where
    A::Item : Clone,
{
    fn clone (
        self: &Self,
    ) -> Self
    {
        self.iter().cloned().collect()
    }
}

mod from_iter;

pub(in crate)
mod into_iter;

pub use self::array_into_iter::*;
mod array_into_iter;

mod try_into;

pub use self::try_from_iter::*;
mod try_from_iter;
