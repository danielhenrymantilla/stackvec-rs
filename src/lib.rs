#![doc(test(attr(deny(warnings))))]
#![doc(test(attr(allow(unused_variables))))]
#![warn(missing_docs)]
#![doc(html_root_url = "https://docs.rs/stackvec/0.1.1")]

#![cfg_attr(feature = "nightly",
    feature(trusted_len, exact_size_is_empty)
)]

// #![cfg_attr(feature = "nightly",
//     feature(try_from)
// )]

#![cfg_attr(feature = "nightly",
    feature(external_doc)
)]
#![cfg_attr(feature = "nightly",
    doc(include = "../README.md")
)]
#![cfg_attr(not(feature = "nightly"),
    doc = "See [crates.io](https://crates.io/crates/stackvec)"
)]
#![cfg_attr(not(feature = "nightly"),
    doc = "for more info about this crate."
)]

use ::std::*;

/// Module to bring the most important items into scope
/// ```rust
/// # #![allow(unused_imports)]
/// extern crate stackvec; use ::stackvec::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        StackVec,
        ArrayIntoIter,
        TryInto,
        TryFromIterator,
        TryCollect,
    };

    pub use ::std::iter::FromIterator;
}

pub use self::array::Array;
mod array;

pub mod error;
use self::error::*;

mod stackvec;
pub use self::stackvec::*;
pub use self::into_iter::Iter as IntoIter;

#[cfg(test)]
mod tests;
