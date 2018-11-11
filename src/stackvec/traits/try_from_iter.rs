use super::*;

use self::iter::FromIterator;

use self::try_into::TryInto;

/// Fallible conversion from an [`Iterable`][`IntoIterator`].
///
/// By implementing [`TryFromIterator`] for a type, you define how it will be
/// created from an [`Iterable`][`IntoIterator`].
/// This is common for types which describe a collection of some kind.
///
/// [`TryFromIterator`]'s 
/// [`try_from_iter`][`::stackvec::traits::TryFromIterator::try_from_iter`]
/// is rarely called explicitly, and is instead
/// used through [`Iterator`]'s [`try_collect`] method.
///
/// See also: [`IntoIterator`].
///
/// [`try_collect`]: `::stackvec::traits::TryCollect::try_collect`
pub trait TryFromIterator<Item>: Sized {
    /// The type returned in the event of a conversion error.
    type Error: ::std::error::Error;

    /// Attempts to perform the conversion.
    fn try_from_iter<Iterable: IntoIterator<Item = Item>> (
        iterable: Iterable
    ) -> Result<Self, Self::Error>;
}

/// [`Iterator`][`Iterator`] [extension trait](
/// https://users.rust-lang.org/t/supertraits-vs-generic-implementations/21266)
/// to extend [`Iterator`]s with a
/// [`.try_collect()`][::stackvec::traits::TryCollect::try_collect] method.
///
/// # Example
///
/// ```rust 
/// extern crate stackvec;
/// 
/// use ::stackvec::prelude::*; // Needed to bring `.try_collect()` into scope.
/// 
/// fn main ()
/// {
///     let array: [_; 3] = [1, 2, 3];
/// 
///     let doubled: [_; 3] = array
///                             .iter()
///                             .map(|&x| 2 * x)
///                             .try_collect() 
///                             .expect("Missing elements to collect")
///     ;
///     assert_eq!(doubled, [2, 4, 6]);
/// } 
/// ```
pub trait TryCollect: Iterator + Sized {
    /// Fallible version of [`collect`][`Iterator::collect`]
    #[inline(always)]
    fn try_collect<Collection> (
        self: Self,
    ) -> Result<Collection, Collection::Error>
    where
        Collection: TryFromIterator<Self::Item>,
    {
        Collection::try_from_iter(self)
    }
}
impl<T: Iterator + Sized> TryCollect for T {}

// pub enum Unreachable {}

// impl fmt::Display for Unreachable {
//     fn fmt (&self, _: &mut fmt::Formatter) -> fmt::Result {
//         unreachable!()
//     }
// }
// impl fmt::Debug for Unreachable {
//     fn fmt (&self, _: &mut fmt::Formatter) -> fmt::Result {
//         unreachable!()
//     }
// }
// impl ::std::error::Error for Unreachable {}

// impl<Item, T: iter::FromIterator<Item>> TryFromIterator<Item> for T {
//     type Error = Unreachable;

//     #[inline(always)]
//     fn try_from_iter<Iterable: IntoIterator<Item = Item>> (
//         iterable: Iterable,
//     ) -> Result<Self, Self::Error>
//     {
//         Ok(Self::from_iter(iterable))
//     }
// }

impl<A: Array> TryFromIterator<A::Item> for A {
    type Error = <StackVec<A> as super::try_into::TryInto<A>>::Error;

    #[inline(always)]
    fn try_from_iter<Iterable: IntoIterator<Item = A::Item>> (
        iterable: Iterable,
    ) -> Result<Self, Self::Error>
    {
        let stackvec = StackVec::from_iter(iterable);
        stackvec.try_into()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn it_works_with_enough_elements ()
    {
        let array: [_; 15] =
            (0 .. 15)
            .try_collect()
            .expect("Missing elements to collect");
        assert_eq!(
            array,
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
        )
    }

    #[test]
    #[should_panic]
    fn it_fails_with_missing_elements ()
    {
        let _: [_; 15] =
            (0 .. 10)
            .try_collect()
            .expect("Missing elements to collect");
    }
}
