use std::fmt::Display;

/// Possible QuadTree errors. May be returned from any QuadTree method that
/// returns a `Result` type.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Error {
    OutOfBounds,
    CannotMakeBbox,
    CannotFindSubNode,
    NoneInRadius,
    InvalidDistance,
    Empty,
    CannotCastInfinity,
    CalcMethodNotSet,
    UnsupportedGeometry,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}
