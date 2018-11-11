//! Example to see how "human-readable" the collection error message is.
// $ cargo run --example try_collect_panic

#![allow(unused_variables)]

extern crate stackvec; use ::stackvec::prelude::*;

fn main ()
{
    let array: [_; 3] = [1, 2, 3];

    let doubled: [_; 5] = array
                            .iter()
                            .map(|&x| 2 * x)
                            .try_collect()
                            .expect("Missing elements to collect")
    ;
}
