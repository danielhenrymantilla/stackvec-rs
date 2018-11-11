//! The crate's [errors][`::std::error::Error`].

use super::*;

use ::std::error::Error;

/// Error returned by [`StackVec::try_push`][`::stackvec::StackVec::try_push]
/// method.
#[derive(Clone, Copy, Debug)]
pub struct OutOfCapacityError<T>(pub T);

impl<T> fmt::Display for OutOfCapacityError<T> {
    fn fmt (
        self: &Self,
        stream: &mut fmt::Formatter,
    ) -> fmt::Result
    {
        fmt::Display::fmt(
            "Attempted to add an element to a full StackVec",
            stream
        )
    }
}

impl<T: fmt::Debug> ::std::error::Error for OutOfCapacityError<T> {
    fn description (
        self: &Self,
    ) -> &str
    {
        "Attempted to add an element to a full StackVec"
    }
}


/// Error returned by
/// [`StackVec::try_into`][`::stackvec::traits::TryInto::try_into]
/// method.
#[derive(Clone, Copy, Debug)]
pub struct IncompleteArrayError;

impl fmt::Display for IncompleteArrayError {
    fn fmt (
        self: &Self,
        stream: &mut fmt::Formatter,
    ) -> fmt::Result
    {
        fmt::Display::fmt(self.description(), stream)
    }
}

impl ::std::error::Error for IncompleteArrayError {
    fn description (
        self: &Self,
    ) -> &str
    {
        concat!(
            "Cannot build an incomplete array.",
        )
    }
}

/// Error used generic-wise to extend fake fallible operations from unfallible
/// ones.
#[derive(Debug)]
pub enum UnreachableError {}

impl UnreachableError {
    /// This error is unconstructible and can thus safely be seen as anything.
    pub fn unreachable (&self) -> ! { match *self {} }
}

impl fmt::Display for UnreachableError {
    fn fmt (&self, _: &mut fmt::Formatter) -> fmt::Result {
        self.unreachable()
    }
}

impl ::std::error::Error for UnreachableError {
    fn description (&self) -> &str {
        self.unreachable()
    }
}