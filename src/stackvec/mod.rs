use super::*;

pub use self::traits::*;
mod traits;

/// Like a [`Vec`], but inlined / "stored in the stack"
///
/// It is backed by a partially uninitialised [`array`], that keeps track of
/// its initialised / uninitialised slots by using a `len: usize` field.
///
/// **Its capacity is the length of the backing [`array`]**, and is thus
/// (statically) fixed within its type: see [the `Array` trait].
/// Only an [`array`] that implements [the `Array` trait] can be used as a
/// backing array.
///
/// It can be constructed:
///
/// - either by hand with [its constructor][`StackVec::new`]:
///   ```rust
///   # use ::stackvec::prelude::*;
///   let empty_vec: StackVec<[i32; 16]>
///       = Default::default();
///   assert_eq!(
///     empty_vec.as_slice(),
///     &[],
///   );
///   ```
///
/// - or by [collecting][`Iterator::collect`]
///   an [iterable][`iter::IntoIterator`]:
///   ```rust
///   # use ::stackvec::prelude::*;
///   let vec: StackVec<[i32; 16]>
///       = (0 .. 5)
///             .filter(|&x| x % 2 == 0)
///             .map(|x| x * x)
///             .collect();
///   assert_eq!(
///       vec.as_slice(),
///       &[0, 4, 16],
///   );
///   ```
///
/// [`array`]: https://doc.rust-lang.org/std/primitive.array.html
/// [the `Array` trait]: `stackvec::Array`
pub struct StackVec<A: Array> {
    array: mem::ManuallyDrop<A>,
    len: usize,
}

impl<A: Array> Default for StackVec<A> {
    /// Default constructor: new empty [`StackVec`]
    #[inline(always)]
    fn default () -> Self
    {
        debug_assert!(Self::CAPACITY <= isize::MAX as usize);
        StackVec {
            len: 0,
            array: mem::ManuallyDrop::new(unsafe { mem::uninitialized() }),
        }
    }
}

impl<A: Array> StackVec<A> {
    /// The (statically) fixed capacity of the [`StackVec`]
    pub const CAPACITY: usize = A::LEN;

    /// The (statically) fixed capacity of the [`StackVec`]
    #[inline]
    pub fn capacity (&self) -> usize { Self::CAPACITY }

    /// Constructor: alias for [`Stackvec::default`](
    /// struct.StackVec.html#impl-Default)
    #[inline(always)]
    pub fn new () -> Self
    {
        Self::default()
    }

    /// Attempts to push a `value` into the [`StackVec`].
    ///
    /// If it is full, it fails returning the given `value` wrapped in
    /// a `Err(OutOfCapacityError(value))`
    #[inline]
    pub fn try_push (
        self: &mut Self,
        value: A::Item,
    ) -> Result<(), OutOfCapacityError<A::Item>>
    {
        debug_assert!(self.len <= Self::CAPACITY);
        if self.len == Self::CAPACITY {
            Err(OutOfCapacityError(value))
        } else {
            unsafe {
                self.push_unchecked(value)
            };
            Ok(())
        }
    }

    /// Pushes the given `value` into the [`StackVec`] if there is room for it,
    /// else it does nothing.
    ///
    /// This has the same semantics as
    /// `let _ = stackvec.try_push(value);`
    /// but may be more efficient.
    #[inline]
    pub fn push_or_ignore (
        self: &mut Self,
        value: A::Item,
    )
    {
        debug_assert!(self.len <= Self::CAPACITY);
        if self.len < Self::CAPACITY {
            unsafe { self.push_unchecked(value) }
        };
    }

    /// Pushes the given `value`
    /// into the [`StackVec`], without any kind of bound-checking whatsoever.
    ///
    /// This is generally not recommended, use with caution!
    /// 
    /// For a safe alternative use
    /// [`self.try_push().expect("stackvec cannot be full")`]
    /// [`StackVec::try_push`]
    ///
    /// # Safety
    ///
    /// The assertion that `stackvec.len() < stackvec.capacity()` must hold
    /// for the operation to be safe.
    #[inline]
    pub unsafe fn push_unchecked (
        self: &mut Self,
        value: A::Item,
    )
    {
        debug_assert!(self.len < Self::CAPACITY); // implicit assertion
        ptr::write(
            self.array.as_mut_ptr()
                .offset(self.len as isize),
            value,
        );
        self.len += 1;
    }

    /// Removes `value` and returns `Some(value)`, where `value` is the last
    /// element of the non-empty [`StackVec`], else it just returns `None`.
    #[inline]
    pub fn pop (
        self: &mut Self,
    ) -> Option<A::Item>
    {
        debug_assert!(self.len <= Self::CAPACITY);
        if self.len > 0 {
            self.len -= 1;
            Some(
                unsafe {
                    ptr::read(
                        self.array.as_ptr()
                            .offset(self.len as isize),
                    )
                }
            )
        } else {
            None
        }
    }

    /// Shortens the [`StackVec`], keeping the first `new_len` elements and
    /// dropping the rest.
    ///
    /// If `new_len` is greater than the current length, this has no effect.
    ///
    /// This has the same semantics as
    /// `(new_len .. stackvec.len()).for_each(|_| { stackvec.pop(); })`
    /// but may be more efficient.
    #[inline]
    pub fn truncate (
        self: &mut Self,
        new_len: usize,
    )
    {
        for new_len in Iterator::rev(new_len .. self.len) {
            self.len = new_len;
            unsafe {
                ptr::drop_in_place(
                    self.array.as_mut_ptr()
                        .offset(new_len as isize)
                );
            };
        };
    }

    /// Clears the [`StackVec`], removing all the values.
    ///
    /// This is exactly the same as `stackvec.truncate(0)`.
    #[inline]
    pub fn clear (
        self: &mut Self,
    )
    {
        self.truncate(0)
    }

    /// Extracts a slice containing the entire [`StackVec`].
    ///
    /// Equivalent to `&stackvec[..]`.
    #[inline]
    pub fn as_slice (
        self: &Self,
    ) -> &[A::Item]
    {
        &* self
    }

    /// Extracts a mutable slice of the entire [`StackVec`].
    ///
    /// Equivalent to `&mut stackvec[..]`.
    #[inline]
    pub fn as_mut_slice (
        self: &mut Self,
    ) -> &mut [A::Item]
    {
        &mut* self
    }

    /// Returns `true` iff the [`StackVec`] is empty
    /// (`self.len() == 0`)
    #[inline]
    pub fn is_empty (
        self: &Self,
    ) -> bool
    {
        self.len() == 0
    }

    /// Returns `true` iff the [`StackVec`] is full
    /// (`self.len() == self.capacity()`)
    #[inline]
    pub fn is_full (
        self: &Self,
    ) -> bool
    {
        debug_assert!(self.len <= Self::CAPACITY);
        self.len() == Self::CAPACITY
    }

    /// Fills the [`StackVec`] with the different values created by the
    /// given `factory`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ::stackvec::prelude::*;
    /// let vec = StackVec::<[Option<String>; 64]>::
    ///     default()
    ///         .fill_using(|| None);
    /// ```
    ///
    /// ```rust
    /// # use ::stackvec::prelude::*;
    /// let vec = StackVec::<[String; 64]>::
    ///     default()
    ///         .fill_using(Default::default);
    /// ```
    ///
    /// ```rust
    /// # use ::stackvec::prelude::*;
    /// let s = String::from("!");
    /// let vec = StackVec::<[String; 64]>::
    ///     from_iter(
    ///         ["Hello", "world"]
    ///             .into_iter()
    ///             .map(String::from)
    ///     ).fill_using(|| s.clone());
    /// ```
    ///
    /// See [`StackVec<Array>::try_into::<Array>`][`stackvec::traits::TryInto`]
    /// for more detailed examples.
    #[inline]
    pub fn fill_using (
        self: &mut Self,
        factory: impl FnMut() -> A::Item,
    )
    {
        self.extend(
            iter::repeat_with(factory)
        )
    }
}

impl<A: Array> StackVec<A>
where
    A::Item: Copy,
{
    /// Fills the [`StackVec`] with [copies][`Copy`] of the given `value`.
    ///
    /// # Example
    /// ```rust
    /// # use ::stackvec::prelude::*;
    /// let vec = StackVec::<[&'static str; 64]>::
    ///     from_iter(
    ///         ["Hello", "world"]
    ///             .into_iter()
    ///     ).fill_with("!");
    /// ```
    #[inline]
    pub fn fill_with (
        self: &mut Self,
        value: A::Item,
    )
    {
        self.extend(
            iter::repeat(value)
        )
    }
}

impl<A: Array> Drop for StackVec<A> {
    #[inline]
    fn drop (
        self: &mut Self,
    )
    {
        self.clear()
    }
}

impl<A: Array> ops::Deref for StackVec<A> {
    type Target = [A::Item];

    #[inline]
    fn deref (
        self: &Self,
    ) -> &Self::Target
    {
        unsafe {
            slice::from_raw_parts(
                self.array.as_ptr(),
                self.len,
            )
        }
    }
}

impl<A: Array> ops::DerefMut for StackVec<A> {
    #[inline]
    fn deref_mut (
        self: &mut Self,
    ) -> &mut Self::Target
    {
        unsafe {
            slice::from_raw_parts_mut(
                self.array.as_mut_ptr(),
                self.len,
            )
        }
    }
}

impl<A: Array> fmt::Debug for StackVec<A>
where
    A::Item: fmt::Debug,
{
    fn fmt (
        self: &Self,
        stream: &mut fmt::Formatter,
    ) -> fmt::Result
    {
        try!(fmt::Display::fmt("[", stream));
        let mut iterator = self.iter();
        if let Some(first) = iterator.next() {
            try!(fmt::Debug::fmt(first, stream));
            for x in iterator {
                try!(write!(stream, ", {:?}", x));
            };
        };
        fmt::Display::fmt("]", stream)
    }
}
