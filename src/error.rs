/// Possible QuadTree errors. May be returned from any QuadTree method that
/// returns a `Result` type.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Error {
    OutOfBounds,
    CannotMakeBbox,
    CannotFindSubNode,
    NoneInRadius,
    Empty,
    CannotCastInfinity,
}