use super::*;

impl<A: Array> iter::Extend<A::Item> for StackVec<A> {
    #[inline]
    fn extend<Iterable: IntoIterator<Item = A::Item>> (
        self: &mut Self,
        iterable: Iterable,
    )
    {
        // This is currently the most optimized `extend` implementation,
        // branching-prediction-wise
        let mut len = self.len;
        debug_assert!(len <= Self::CAPACITY);
        if len == Self::CAPACITY {
            return
        };
        unsafe {
            for value in iterable {
                debug_assert!(len < Self::CAPACITY);
                ptr::write(
                    self.array.as_mut_ptr()
                        .offset(len as isize),
                    value,
                );
                len += 1;
                self.len = len;
                if len == Self::CAPACITY { break };
            };
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
