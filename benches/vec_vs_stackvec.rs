// $ cargo +nightly bench --features nightly
#![cfg(all(test, feature = "nightly"))]
#![feature(test)]

extern crate stackvec;
use ::stackvec::prelude::*;

extern crate test;
use ::test::{
	Bencher,
};

#[bench]
fn stackvec_extend_by_ref (benchmark: &mut Bencher)
{
    benchmark.iter(|| {
        for n in 0 .. (0x400 - 8) {
        	let mut vec = StackVec::<[_; 0x400]>::new();
        	vec.extend(
        		Iterator::chain(
			    	(0 .. n).map(|x| x * x),
			    	b"StackVec".iter().map(|&b| b as u16),
				)
				.by_ref()
	        );
	    }
    });
}

#[bench]
fn vec_extend_by_ref (benchmark: &mut Bencher)
{
    benchmark.iter(|| {
        for n in 0 .. (0x400 - 8) {
        	let mut vec = Vec::with_capacity(0x400);
	        vec.extend(
	        	Iterator::chain(
			    	(0 .. n).map(|x| x * x),
			    	b"StackVec".iter().map(|&b| b as u16),
				)
				.by_ref()
	        );
    	}
    });
}

#[bench]
fn stackvec_extend (benchmark: &mut Bencher)
{
    benchmark.iter(|| {
        for n in 0 .. (0x400 - 8) {
        	let mut vec = StackVec::<[_; 0x400]>::new();
        	vec.extend(
        		Iterator::chain(
			    	(0 .. n).map(|x| x * x),
			    	b"StackVec".iter().map(|&b| b as u16),
				)
	        );
	    }
    });
}

#[bench]
fn vec_extend (benchmark: &mut Bencher)
{
    benchmark.iter(|| {
        for n in 0 .. (0x400 - 8) {
        	let mut vec = Vec::with_capacity(0x400);
	        vec.extend(
	        	Iterator::chain(
			    	(0 .. n).map(|x| x * x),
			    	b"StackVec".iter().map(|&b| b as u16),
				)
	        );
    	}
    });
}

#[bench]
fn stackvec_from_iter (benchmark: &mut Bencher)
{
    benchmark.iter(|| {
        for n in 0 .. (0x400 - 8) {
        	let _vec = StackVec::<[_; 0x400]>::from_iter(
        		Iterator::chain(
			    	(0 .. n).map(|x| x * x),
			    	b"StackVec".iter().map(|&b| b as u16),
				)
	        );
	    }
    });
}

#[bench]
fn vec_from_iter (benchmark: &mut Bencher)
{
    benchmark.iter(|| {
        for n in 0 .. (0x400 - 8) {
	        let _vec = Vec::from_iter(
	        	Iterator::chain(
			    	(0 .. n).map(|x| x * x),
			    	b"StackVec".iter().map(|&b| b as u16),
				)
	        );
    	}
    });
}

#[bench]
fn array_from_iter (benchmark: &mut Bencher)
{
    benchmark.iter(|| {
        for n in 0 .. (0x400 - 8) {
        	let _array = <[_; 0x400]>::try_from_iter(
        		Iterator::chain(
			    	(0 .. n).map(|x| x * x),
			    	b"StackVec".iter().map(|&b| b as u16),
				)
	        );
	    }
    });
}
