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
			len: A::COUNT,
		}
	}
}

pub trait ArrayIntoIter: Array {
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
