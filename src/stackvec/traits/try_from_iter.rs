use super::*;

use self::iter::FromIterator;

use self::try_into::TryInto;

pub trait TryFromIterator<Item>: Sized {
	type Error: ::std::error::Error;

	fn try_from_iter<Iterable: IntoIterator<Item = Item>> (
		iterable: Iterable
	) -> Result<Self, Self::Error>;
}

pub trait TryCollect: Iterator + Sized {
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
// 	fn fmt (&self, _: &mut fmt::Formatter) -> fmt::Result {
// 		unreachable!()
// 	}
// }
// impl fmt::Debug for Unreachable {
// 	fn fmt (&self, _: &mut fmt::Formatter) -> fmt::Result {
// 		unreachable!()
// 	}
// }
// impl ::std::error::Error for Unreachable {}

// impl<Item, T: iter::FromIterator<Item>> TryFromIterator<Item> for T {
// 	type Error = Unreachable;

// 	#[inline(always)]
// 	fn try_from_iter<Iterable: IntoIterator<Item = Item>> (
// 		iterable: Iterable,
// 	) -> Result<Self, Self::Error>
// 	{
// 		Ok(Self::from_iter(iterable))
// 	}
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
