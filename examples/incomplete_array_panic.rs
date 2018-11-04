//! Example to see how "human-readable" the collection error message is.

extern crate stackvec;

use ::stackvec::prelude::*;

fn main ()
{
	let _: [_; 25] =
		(0 .. 20)
		.try_collect()
		.expect("Missing elements to collect");
}