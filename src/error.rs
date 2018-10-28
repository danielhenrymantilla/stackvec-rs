use super::*;

#[derive(Debug)]
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

#[derive(Debug)]
pub enum UnreachableError {}

impl UnreachableError {
    pub fn into<T> (self) -> T { unreachable!() }
}

impl fmt::Display for UnreachableError {
    fn fmt (&self, _: &mut fmt::Formatter) -> fmt::Result {
        unreachable!()
    }
}

impl ::std::error::Error for UnreachableError {
    fn description (&self) -> &str {
        unreachable!()
    }
}