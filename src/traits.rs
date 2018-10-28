use super::*;

impl<A: Array> iter::Extend<A::Item> for StackVec<A> {
    #[inline]
    fn extend<Iterable: IntoIterator<Item = A::Item>> (
        self: &mut Self,
        iterable: Iterable,
    )
    {
        debug_assert!(self.len <= Self::CAPACITY);
        // This is currently the most optimized `extend` implementation,
        // branching-prediction-wise
        if self.len == Self::CAPACITY {
            return
        };
        for value in iterable {
            let mut len = self.len;
            unsafe {
                debug_assert!(len < Self::CAPACITY);
                ptr::write(
                    self.array.as_mut_ptr()
                        .offset(len as isize),
                    value,
                );
                len += 1;
                self.len = len;
            };
            if len == Self::CAPACITY { break };
        };

        // // This version was less optimized:
        // let mut iterator = iterable.into_iter();
        // let mut len = self.len;
        // while len < Self::CAPACITY {
        //     if let Some(value) = iterator.next() {
        //         unsafe {
        //             ptr::write(
        //                 self.array.as_mut_ptr()
        //                     .offset(len as isize),
        //                 value,
        //             );
        //             len += 1;
        //         }
        //     } else {
        //         break
        //     };
        // };
        // self.len = len;

        // // And this one even worse o_O
        // iterable.into_iter()
        //     .take(Self::CAPACITY - self.len)
        //     .for_each(|value| unsafe {
        //         self.push_unchecked(value)
        //     })
    }
}

impl<A: Array> iter::FromIterator<A::Item> for StackVec<A> {
    #[inline(always)]
    fn from_iter<Iterable: IntoIterator<Item = A::Item>> (
        iterable: Iterable,
    ) -> Self
    {
        let mut slf = Self::new();
        slf.extend(iterable);
        slf
    }
}

pub use self::into_iter::Iter as StackVecIter;
mod into_iter {
    use super::*;

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

    impl<A: Array> ExactSizeIterator for Iter<A> {
        #[inline]
        fn len (
            self: &Self,
        ) -> usize
        {
            self.stackvec.len - self.start
        }

        #[cfg(feature = "exact_size_is_empty")]
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

        #[inline]
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
}
