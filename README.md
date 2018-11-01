# [stackvec-rs][Documentation] (version 0.0.2)
A rust crate to use stack-allocated vectors (to improve performance and/or when there is no std)

* [Documentation (WIP)][Documentation]
* [Repository]
* [crates.io]

## ⚠️⚠️ Warning: `unsafe` is used ⚠️⚠️
And hasn't been thoroughly tested yet. It is thus ill-suited for production. Use at your own risk.

Since [stackvec-rs][Documentation] provides very similar functionality to the more mature [`arrayvec`](https://docs.rs/arrayvec/0.4.7/arrayvec/), you should use that crate until [stackvec-rs][Documentation] is mature enough (version 0.1.0 or even 1.0.0)


## Motivation
Rust [stack/inline arrays](https://doc.rust-lang.org/std/primitive.array.html) don't implement 2 very useful [iterator-related](https://doc.rust-lang.org/std/iter) [interfaces](https://doc.rust-lang.org/stable/std/iter/#traits):

1. `IntoIterator<Item = T> for [T; n]`

  	* Allows using `.into_iter()` instead of `.iter().cloned()` (which, by the way, can only be used when `T: Clone`, and requires cloning, which may be expensive)
   	* ```rust
   	  #[derive(Hash, PartialEq, Eq)]
   	  struct NoClone {
   	  	/* ... */
   	  }
   	  let array: [NoClone, 15] = [ /* ... */ ];
   	  let set: ::std::collections::HashSet =
   	  	array()
   	  	.into_iter() // Error
   	  	.collect()
   	  ;
   	  assert!(!set.is_empty());
	  ```

1. `FromIterator for [T; n]`
  	* Allows using [`.collect()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect)
  	* Since it is unsound to have an incomplete array, the collecting fails when the iterator does not have enough elements to fill the array. Thus, since it is a fallible action; there is a new [`TryFromIterator`] trait.
  	* ```rust
	  let mut numbers_iterator = (3 .. 10).into_iter();
	  let array: [u8; 7] = 
	  	numbers_iterator
	  	.try_collect()  // Attempt to collect into the array. This can fail...
	  	.unwrap() // ...since there needs to be at least 7 elements.
	  ;
	  assert_eq!(
	  	array,
	  	[3, 4, 5, 6, 7, 8, 9],
	  ); 
	  ```

The reason for that is that both interfaces need a structure being able to hold
the partially iterated state: i.e., incomplete arrays. Those have (statically-allocated) memory that might not be initialized: so they are, in a way, like [`Vec`]tors (except for the fact that their (initial) capacity is fixed and cannot be changed)

That's why having those nice [iterator](https://doc.rust-lang.org/std/iter) [interfaces](https://doc.rust-lang.org/stable/std/iter/#traits) require writing down a cell-accurate memory ownership management logic very similar to [`Vec`]'s : hence the [`StackVec`].

### Bonus
By exposing the underlying [`StackVec`] needed by the aformentioned interfaces, we get full access to a stack-allocated [`Vec`], which can also be useful on its own, since it avoids heap allocation:

* the heap is a mutable global state and in multi-threaded environments locks are involved,

* it may require (slow) system allocation

* [heap allocation is not always available](https://doc.rust-lang.org/1.7.0/book/no-stdlib.html)

### Disclaimer
The performance gain (from using [`StackVec`] instead of [`Vec`]) is not always guaranteed, since:

1. [`Vec`] is the cornerstone of Rust's std library collection and has extremely efficient code written so that LLVM can easily optimize its usage

1. Rust's [allocator](http://smallcultfollowing.com/babysteps/blog/2014/11/14/allocators-in-rust/) is also incredibly well optimised so the performance penalties from bins management and system allocations (and the locks in a multi-threaded environment) are quite well amortized on average.

#### [`Vec`] vs [`StackVec`] basic benchmark
```sh
$ cargo +nightly bench --features nightly

test stackvec_extend        ... bench:     364,517 ns/iter (+/- 29,075)
test stackvec_extend_by_ref ... bench:     361,498 ns/iter (+/- 10,230)
test vec_extend             ... bench:      69,866 ns/iter (+/- 4,975)
test vec_extend_by_ref      ... bench:     880,585 ns/iter (+/- 17,259)
```

## Usage
  1. Add this line to your `Cargo.toml` (under `[dependencies]`):
  ```toml
  stackvec = "0.0.2"
  ```

  1. Add this to your `.rs` code:
  ```rust
  extern crate stackvec;
  ```

### Examples

See the [source files for the examples](https://github.com/danielhenrymantilla/stackvec-rs/tree/master/)

You can run each example (`example_name.rs`) with:
```sh
$ cargo run --example example_name
```

# WIP
  1. [Documentation]


  1. [`into_iter`]/[`IntoIterator`] and [`try_collect`]/[`TryFromIterator`] for [`Array`]

  1. improve code testing coverage

  1. More [`Vec`]-like [methods](https://docs.rs/stackvec/0.0.2/stackvec/struct.StackVec.html#methods)

[comment]: # (==== LINKS ====)

[Repository]: https://github.com/danielhenrymantilla/stackvec-rs
[Documentation]: https://docs.rs/stackvec/0.0.2/
[crates.io]: https://crates.io/crates/stackvec
[`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
[`IntoIterator`]: https://doc.rust-lang.org/std/iter/trait.IntoIterator.html
[`into_iter`]: https://doc.rust-lang.org/std/iter/trait.IntoIterator.html#tymethod.into_iter
[`FromIterator`]: https://doc.rust-lang.org/std/iter/trait.FromIterator.html
[`from_iter`]: https://doc.rust-lang.org/std/iter/trait.FromIterator.html#tymethod.from_iter
[`StackVec`]: https://docs.rs/stackvec/0.0.2/stackvec/struct.StackVec.html
[`TryFromIterator`]: #
[`try_collect`]: #
