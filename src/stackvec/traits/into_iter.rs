use super::*;

/// An iterator that moves out of a [`StackVec`].
///
/// This `struct` is created by the `into_iter` method (provided
/// by the [`IntoIterator`] trait).
pub struct Iter<A: Array> {
    stackvec: StackVec<A>,
    start: usize,
}

impl<A: Array> Drop for Iter<A> {
    fn drop (
        self: &mut Self,
    )
    {
        let len = self.stackvec.len;
        self.stackvec.len = 0;
        for i in self.start .. len {
            unsafe {
                ptr::drop_in_place(
                    self.stackvec
                        .array.as_mut_ptr()
                        .offset(i as isize)
                );
            };
        };
    }
}

impl<A: Array> Iterator for Iter<A> {
    type Item = A::Item;

    #[inline]
    fn next (
        self: &mut Self,
    ) -> Option<Self::Item>
    {
        let start = self.start;
        if start < self.stackvec.len {
            self.start = start + 1;
            Some(unsafe {
                ptr::read(
                    self.stackvec
                        .array.as_ptr()
                        .offset(start as isize),
                )
            })
        } else {
            None
        }
    }

    #[inline]
    fn size_hint (
        self: &Self,
    ) -> (usize, Option<usize>)
    {
        let size = self.stackvec.len - self.start;
        (size, Some(size))
    }
}

impl<A: Array> iter::FusedIterator for Iter<A> {}

#[cfg(feature = "nightly")]
unsafe impl<A: Array> iter::TrustedLen for Iter<A> {}

impl<A: Array> ExactSizeIterator for Iter<A> {
    #[inline]
    fn len (
        self: &Self,
    ) -> usize
    {
        self.stackvec.len - self.start
    }

    #[cfg(feature = "nightly")]
    #[inline]
    fn is_empty (
        self: &Self,
    ) -> bool
    {
        self.stackvec.len == self.start
    }
}

impl<A: Array> DoubleEndedIterator for Iter<A> {
    #[inline]
    fn next_back (
        self: &mut Self,
    ) -> Option<Self::Item>
    {
        let last = self.stackvec.len.saturating_sub(1);
        if self.start <= last {
            self.stackvec.len = last;
            Some(unsafe {
                ptr::read(
                    self.stackvec
                        .array.as_ptr()
                        .offset(last as isize),
                )
            })
        } else {
            None
        }

    }
}

impl<A: Array> IntoIterator for StackVec<A> {
    type Item = A::Item;

    type IntoIter = Iter<A>;

    #[inline(always)]
    fn into_iter (
        self: Self,
    ) -> Self::IntoIter
    {
        Iter {
            stackvec: self,
            start: 0,
        }
    }
}
