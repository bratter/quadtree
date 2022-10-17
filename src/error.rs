#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Error {
    OutOfBounds,
    CannotMakeBbox,
    NoneInRadius,
    Empty,
}