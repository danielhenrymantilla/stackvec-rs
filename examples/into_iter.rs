// $ cargo run --example into_iter

#![allow(unused_variables)]

extern crate stackvec; use ::stackvec::prelude::*;

fn main ()
{
    // An array of vectors (potentially expensive to clone)
    let vecs_array = [
        vec![1, 2, 3, 4],
        vec![],
        vec![5, 6],
    ];

    // Collect / chain all the vectors together
    let flattened: Vec<u8> = vecs_array
                                .into_iter()
                                .flatten()
                                .collect()
    ;
    assert_eq!(flattened, vec![1, 2, 3, 4, 5, 6]);
}