use super::*;

impl<A: Array> From<A> for StackVec<A>
{
    #[inline(always)]
    fn from (
        array: A,
    ) -> StackVec<A>
    {
        StackVec {
            array: mem::ManuallyDrop::new(array),
            len: A::LEN,
        }
    }
}

/// Grants [`Array`]s an [`.into_iter()`][`ArrayIntoIter::into_iter`]
/// method to almost seamlessly use [`array`]s
/// as [by-owned-value iterators][`IntoIterator`].
///
/// # Example
/// ```rust
/// # use ::stackvec::prelude::*;
/// let array: [_; 2] = [
///     vec![1, 2, 3, 4],
///     vec![5, 6],
/// ];
/// let flattened: Vec<u8> = array
///                             .into_iter()
///                             .flatten()
///                             .collect();
/// assert_eq!(flattened, &[1, 2, 3, 4, 5, 6]);
/// ```
///
/// [`array`]: https://doc.rust-lang.org/std/primitive.array.html
pub trait ArrayIntoIter: Array {
    /// Consumes the given [`Array`] and its contents and returns a
    /// [by-owned-value iterator][`IntoIterator`].
    #[inline(always)]
    fn into_iter (
        self: Self,
    ) -> crate::IntoIter<Self>
    {
        StackVec::from(self).into_iter()
    }
}
impl<A: Array> ArrayIntoIter for A {}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[derive(PartialEq, Eq, Hash)]
    struct NoClone;

    #[test]
    fn array_into_iter ()
    {
        let array = [NoClone, NoClone, NoClone, NoClone];
        let set = ::std::collections::HashSet::<NoClone>::from_iter(
            array.into_iter()
        );
        assert!(!set.is_empty());
    }
}
