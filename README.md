# `[stack;vec]`

**A rust crate to use stack-allocated vectors (to improve performance and/or when there is no std)**

[![Repository](https://img.shields.io/badge/repository-GitHub-brightgreen.svg)][Repository] [![Latest version](https://img.shields.io/crates/v/stackvec.svg)][crates.io] [![Documentation](https://docs.rs/stackvec/badge.svg)][Documentation]

[![Travis-CI Status](https://travis-ci.org/danielhenrymantilla/stackvec-rs.svg?branch=master)](https://travis-ci.org/danielhenrymantilla/stackvec-rs)
[![Test code coverage](https://codecov.io/gh/danielhenrymantilla/stackvec-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/danielhenrymantilla/stackvec-rs)
[![License](https://img.shields.io/crates/l/stackvec.svg)](https://github.com/danielhenrymantilla/stackvec-rs#license)

## ⚠️⚠️ Warning: `unsafe` is used ⚠️⚠️
However, it should be noted that [special attention](https://codecov.io/gh/danielhenrymantilla/stackvec-rs) has been paid to array indices arithmetic (overflow/underflow), since they are, after all, pointers, and to memory management (no memory leaks or double frees).

The only remaining issue is [Exception safety](https://doc.rust-lang.org/nomicon/exception-safety.html); as of `v0.1.0`, a non fatal panic while [`drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html#tymethod.drop)ping (very rare), should "only" cause memory leaks (preferable to dangling pointers).

It may thus be ill-suited for production. Use at your own risk.

## Motivation
Rust [stack/inline arrays](https://doc.rust-lang.org/std/primitive.array.html) don't implement 2 very useful [iterator-related](https://doc.rust-lang.org/std/iter) [interfaces](https://doc.rust-lang.org/stable/std/iter/#traits):

1. `IntoIterator<Item = T> for [T; n]`

  	* Allows using `.into_iter()` instead of `.iter().cloned()` (which, by the way, can only be used when `T: Clone`, and requires cloning, which may be expensive)
   	* ```rust
      extern crate stackvec; use ::stackvec::prelude::*;

   	  #[derive(Hash, PartialEq, Eq)]
   	  struct NoClone {
   	  	/* ... */
   	  }

      fn main ()
      {
     	  let array: [NoClone; _] = [ /* ... */ ];

          // Recollect the array into another collection: `.into_iter().collect()`
     	  let set: ::std::collections::HashSet =
     	  	array()
     	  	.into_iter() // Would error if not for line 1
     	  	.collect()
     	  ;
      }
	  ```

1. `FromIterator for [T; n]`
  	* Allows using [`.collect()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect)
  	* Since it is unsound to have an incomplete array, the collecting fails when the iterator does not have enough elements to fill the array. Hence the new [`TryFromIterator`] trait (providing [`try_collect`]).
  	* ```rust
      extern crate stackvec; use ::stackvec::prelude::*;

      fn main () {
          const N: usize = 7;
          // Attempt to collect into the array. This can fail
          // since there needs to be at least N elements.
    	  let array: [_; N] = 
    	  	(3 .. 10)
    	  	.try_collect() // Would error if not for line 1
    	  	.expect("Missing elements to collect")
          ;
    	  assert_eq!(
    	  	array,
    	  	[3, 4, 5, 6, 7, 8, 9],
    	  );
      }
	  ```

The reason for that is that both interfaces need a structure being able to hold
the partially iterated state: i.e., incomplete arrays. Those have (statically-allocated) memory that might not be initialized: so they are, in a way, like [`Vec`]tors (except for the fact that their (initial) capacity is fixed and cannot be changed)

That's why having those nice [iterator](https://doc.rust-lang.org/std/iter) [interfaces](https://doc.rust-lang.org/stable/std/iter/#traits) require writing down a cell-accurate memory ownership management logic very similar to [`Vec`]'s : hence the [`StackVec`].

### Bonus
By exposing the underlying [`StackVec`] needed by the aformentioned interfaces, we get full access to a stack-allocated [`Vec`], which can also be useful on its own, since it avoids heap allocation:

* the heap is a mutable global state and in multi-threaded environments locks are involved,

* it may require (slow) system allocation

* [heap allocation is not always available][`no_std`]

### Disclaimer
The performance gain (from using [`StackVec`] instead of [`Vec`]) is not always guaranteed, since:

1. [`Vec`] is the cornerstone of Rust's std library collection and has extremely efficient code written so that LLVM can easily optimize its usage

1. Rust's [allocator](http://smallcultfollowing.com/babysteps/blog/2014/11/14/allocators-in-rust/) is also incredibly well optimised so the performance penalties from bins management and system allocations (and the locks in a multi-threaded environment) are quite well amortized on average.

#### [`Vec`] vs [`StackVec`] basic benchmark
```sh
$ cargo +nightly bench --features nightly

test vec_extend             ... bench:      64,129 ns/iter (+/- 3,069)
test vec_from_iter          ... bench:      65,569 ns/iter (+/- 3,761)
test array_from_iter        ... bench:     358,993 ns/iter (+/- 6,916)
test stackvec_extend        ... bench:     360,105 ns/iter (+/- 17,489)
test stackvec_from_iter     ... bench:     369,585 ns/iter (+/- 40,894)
test stackvec_extend_by_ref ... bench:     374,226 ns/iter (+/- 11,686)
test vec_extend_by_ref      ... bench:     863,362 ns/iter (+/- 32,483)
```

## Usage

- Add this line to your `Cargo.toml` (under `[dependencies]`):
  ```toml
  stackvec = "0.1.0"
  ```
    - Note: By default `stackvec` improves all the arrays with less than 1000 elements. This leads to longer compilation times. If this is an issue, and you don't really plan or using arbitrary-length arrays but at fixed multiples of 100 or powers of 2, you can depend on a "lighter" `stackvec` using the following line instead:
      ```toml
      stackvec = { version="0.1.0", default-features = false }
      ``` 

- Add this to your `.rs` code:
  ```rust
  extern crate stackvec;

  use ::stackvec::prelude::*;
  ```

### Examples

See the [source files for the examples](https://github.com/danielhenrymantilla/stackvec-rs/tree/master/)

You can run each example (`example_name.rs`) with:
```sh
$ cargo run --example example_name
```

# WIP
  1. [Documentation]


  1. [`no_std`] support

  1. More [`Vec`]-like [methods](https://docs.rs/stackvec/0.1.0/stackvec/struct.StackVec.html#methods)

[comment]: # (==== LINKS ====)

[Repository]: https://github.com/danielhenrymantilla/stackvec-rs
[Documentation]: https://docs.rs/stackvec/0.1.0/
[crates.io]: https://crates.io/crates/stackvec

[`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html

[`IntoIterator`]: https://doc.rust-lang.org/std/iter/trait.IntoIterator.html
[`into_iter`]: https://doc.rust-lang.org/std/iter/trait.IntoIterator.html#tymethod.into_iter

[`FromIterator`]: https://doc.rust-lang.org/std/iter/trait.FromIterator.html
[`from_iter`]: https://doc.rust-lang.org/std/iter/trait.FromIterator.html#tymethod.from_iter

[`StackVec`]: https://docs.rs/stackvec/0.1.0/stackvec/struct.StackVec.html

[`TryFromIterator`]: https://docs.rs/stackvec/0.1.0/stackvec/traits/trait.TryFromIterator.html
[`try_collect`]: https://docs.rs/stackvec/0.1.0/stackvec/trait.TryCollect.html#method.try_collect

[`no_std`]: https://doc.rust-lang.org/1.7.0/book/no-stdlib.html
