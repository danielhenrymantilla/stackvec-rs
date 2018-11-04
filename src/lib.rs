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
    doc = "See [crates.io](https://crates.io) for more info about this crate."
)]

use ::std::*;

pub mod prelude {
    pub use super::{
        StackVec,
        TryFromIterator,
        TryCollect,
        ArrayIntoIter,
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
