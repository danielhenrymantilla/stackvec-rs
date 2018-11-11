// $ cargo run --example try_collect

#![allow(unused_variables)]

extern crate stackvec; use ::stackvec::prelude::*;

fn main ()
{
    let array: [_; 3] = [1, 2, 3];

    let doubled: [_; 3] = array
                            .iter()
                            .map(|&x| 2 * x)
                            .try_collect()
                            .expect("Missing elements to collect")
    ;
    assert_eq!(doubled, [2, 4, 6]);
}
