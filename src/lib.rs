#![cfg_attr(feature = "nightly",
    feature(external_doc))
]
#![cfg_attr(feature = "nightly",
    doc(include = "../README.md"))
]

use ::std::*;

pub mod prelude {
    pub use super::{
        StackVec,
    };

    pub use ::std::iter::FromIterator;
}

mod array;
pub use self::array::Array;

pub mod error;
use self::error::*;

pub struct StackVec<A: Array> {
    array: mem::ManuallyDrop<A>,
    len: usize,
}

impl<A: Array> Default for StackVec<A> {
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
    pub const CAPACITY: usize = A::N;

    #[inline(always)]
    pub fn new () -> Self
    {
        Self::default()
    }

    #[inline(always)]
    unsafe fn push_unchecked (
        self: &mut Self,
        value: A::Item,
    )
    {
        ptr::write(
            self.array.as_mut_ptr()
                .offset(self.len as isize),
            value,
        );
        self.len += 1;
    }

    #[inline(always)]
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

    #[inline(always)]
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

    #[inline]
    pub fn truncate (
        self: &mut Self,
        len: usize,
    )
    {
        /* for _ in len .. self.len() { self.pop() } */
        let end = self.len;
        if len < end {
            self.len = len;
            for i in len .. end {
                unsafe {
                    ptr::drop_in_place(
                        self.array.as_mut_ptr()
                            .offset(i as isize)
                    );
                };
            };
        };
    }

    #[inline]
    pub fn clear (
        self: &mut Self,
    )
    {
        self.truncate(0)
    }

    #[inline(always)]
    pub fn as_slice (
        self: &Self,
    ) -> &[A::Item]
    {
        &* self
    }

    #[inline(always)]
    pub fn as_mut_slice (
        self: &mut Self,
    ) -> &mut [A::Item]
    {
        &mut* self
    }
}

impl<A: Array> Drop for StackVec<A> {
    fn drop (
        self: &mut Self,
    )
    {
        self.clear()
    }
}

impl<A: Array> ops::Deref for StackVec<A> {
    type Target = [A::Item];

    #[inline(always)]
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
    #[inline(always)]
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

pub mod traits;
