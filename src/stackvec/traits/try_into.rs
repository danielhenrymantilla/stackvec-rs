use super::*;

// #[cfg(feature = "nightly")]
// pub use std::convert::TryInto;
// #[cfg(not(feature = "nightly"))]

/// An attempted conversion that consumes `self`,
/// which may or may not be expensive.
pub trait TryInto<Dst> {
	/// The type returned in the event of a conversion error.
	type Error: ::std::error::Error;

	/// Attempts to perform the conversion.
	fn try_into (
		self: Self
	) -> Result<Dst, Self::Error>;
}

/// Try to promote a [`StackVec`] to a full [`array`]
///
/// # Example
/// ```
/// # use ::stackvec::prelude::*;
/// /// Objective: build an array of `Option<String>`
/// /// [Some("Hello"), Some("world!"), None, None, ... , None]
/// type Array = [Option<String>; 64];
///
/// // Since `String` is not `Copy`, `None: Option<String>` isn't either.
/// // Thus, sadly, we cannot do:
/// // let mut array: Array = [None; 64];
/// // array[0] = Some(String::from("Hello"));
/// // array[1] = Some(String::from("world!"));
/// 
/// let mut vec: StackVec<Array>
///     = ["Hello", "world!"] // First 2 values
///         .iter()
///         .map(|&x| Some(String::from(x)))
///         .collect();
///
/// assert_eq!(vec[0].as_ref().unwrap(), "Hello");
/// assert_eq!(vec[1].as_ref().unwrap(), "world!");
/// 
/// assert!(vec.len() < vec.capacity()); // not full
/// // Thus it can't be converted to an array yet
/// // let array: Array = vec.try_into().unwrap(); // conversion would fail
/// 
/// vec.fill_using(|| None); // Fill with `None`s
/// assert_eq!(vec.len(), vec.capacity()); // now it is full
/// let array: Array = vec.try_into().unwrap(); // conversion can now be successful
/// ```
///
/// ```
/// # use ::stackvec::prelude::*;
/// /// Objective: build an array of `String`
/// /// ["Hello", "world!", "", "", ... , ""]
/// type Array = [String; 64];
/// 
/// let mut vec: StackVec<Array>
///     = ["Hello", "world!"] // First 2 values
///         .into_iter()
///         .map(String::from)
///         .collect();
///
/// assert_eq!(vec[0], "Hello");
/// assert_eq!(vec[1], "world!");
/// 
/// assert!(vec.len() < vec.capacity()); // not full
/// // Thus it can't be converted to an array yet
/// // let array: Array = vec.try_into().unwrap(); // conversion would fail
/// 
/// let end = String::from("");
/// vec.fill_using(|| end.clone()); // Fill with clones
/// // or
/// // vec.fill_using(Default::default); // Fill with default value
///
/// assert_eq!(vec.len(), vec.capacity()); // now it is full
/// let array: Array = vec.try_into().unwrap(); // conversion can now be successful
/// ```
///
/// [`array`]: https://doc.rust-lang.org/std/primitive.array.html
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
