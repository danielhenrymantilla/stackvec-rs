use super::*;
use ::std::error::Error as StdError;

// #[cfg(feature = "nightly")]
// pub use std::convert::TryInto;
// #[cfg(not(feature = "nightly"))]

pub trait TryInto<Dst> {
	type Error: ::std::error::Error;

	fn try_into (
		self: Self
	) -> Result<Dst, Self::Error>;
}

#[derive(Clone, Copy, Debug)]
pub struct IncompleteArrayError;

impl fmt::Display for IncompleteArrayError {
	fn fmt (
		self: &Self,
		stream: &mut fmt::Formatter,
	) -> fmt::Result
	{
		fmt::Display::fmt(self.description(), stream)
	}
}

impl ::std::error::Error for IncompleteArrayError {
	fn description (
		self: &Self,
	) -> &str
	{
		concat!(
			"Cannot build an incomplete array.",
		)
	}
}

impl<A: Array> TryInto<A> for StackVec<A>
{
	type Error = IncompleteArrayError;

	#[inline(always)]
	fn try_into (
		self: StackVec<A>,
	) -> Result<A, Self::Error>
	{
		if self.len == Self::CAPACITY {
			let array_ptr: *const mem::ManuallyDrop<A> = &self.array;
			mem::forget(self);
			Ok(mem::ManuallyDrop::into_inner(
				unsafe { ptr::read(array_ptr) }
			))
		} else {
			Err(IncompleteArrayError)
		}
	}
}
