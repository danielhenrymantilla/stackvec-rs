// cargo +nightly bench
#![cfg(nightly)]
#![feature(test)]

extern crate stackvec;
use ::stackvec::prelude::*;

extern crate test;
use ::test::{
	// black_box,
	Bencher,
};

// #[inline(always)] fn black_box<T> (x: T) -> T { x }

#[bench]
fn stackvec_append(benchmark: &mut Bencher)
{
    benchmark.iter(|| {
        for n in 0 .. (0x400 - 6) {
        	let mut vec = StackVec::<[_; 0x400]>::new();
        	vec.extend(
        		Iterator::chain(
			    	(0 .. n).map(|x| x * x),
			    	b"Shrewk".iter().map(|&b| b as u16),
				)
				.by_ref()
	        );
	    }
    });
}

#[bench]
fn vec_append(benchmark: &mut Bencher)
{
    benchmark.iter(|| {
        for n in 0 .. (0x400 - 6) {
        	let mut vec = Vec::with_capacity(0x400);
	        vec.extend(
	        	Iterator::chain(
			    	(0 .. n).map(|x| x * x),
			    	b"Shrewk".iter().map(|&b| b as u16),
				)
				.by_ref()
	        );
    	}
    });
}

#[bench]
fn stackvec_extend(benchmark: &mut Bencher)
{
    benchmark.iter(|| {
        for n in 0 .. (0x400 - 6) {
        	let mut vec = StackVec::<[_; 0x400]>::new();
        	vec.extend(
        		Iterator::chain(
			    	(0 .. n).map(|x| x * x),
			    	b"Shrewk".iter().map(|&b| b as u16),
				)
	        );
	    }
    });
}

#[bench]
fn vec_extend(benchmark: &mut Bencher)
{
    benchmark.iter(|| {
        for n in 0 .. (0x400 - 6) {
        	let mut vec = Vec::with_capacity(0x400);
	        vec.extend(
	        	Iterator::chain(
			    	(0 .. n).map(|x| x * x),
			    	b"Shrewk".iter().map(|&b| b as u16),
				)
	        );
    	}
    });
}