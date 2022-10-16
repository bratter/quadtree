/*
 * Quadtree Package.
 * 
 * Multiple quadtree implementations for various geometries.
 * 
 * TODO: Consumers need an easy time of sending radians into dist - perhaps just to_radians on Point is fine?
 * TODO: I think the way consumers can work this is have a wrapper type that is a datum that for find/knn impls DistEuclidean (should test this)
 * TODO: Distance sorted iterator, retrieve as an iterator
 * TODO: Document or test that retrieve.filter can give intersections or contains; convert retrieve to an iterator?
 * TODO: Write documentation for everything
 * TODO: Force constraints on spherical coords?
 * TODO: Add clear and remove operations to the quadtree trait
 * TODO: Should we use Error semantics for insertion? Probably yes
 * TODO: Build an integer-with-power-2-bounds version?
 * TODO: Should nodes and children be private on the node structs?
 * TODO: Make a PR for the geo crate to add extra euclidean and haversine distance measures for Rect
 *       PR also should include fixing the TODO in https://docs.rs/geo/latest/src/geo/algorithm/contains/rect.rs.html#30-42
 *       This is wrong and should probably use > not >=
 */

mod geom;
mod quadtrees;
mod node;
mod iter;

use geom::*;
use node::*;
use iter::*;

// Export the quadtree traits/constants, and implementations
pub use quadtrees::*;
pub use quadtrees::point::*;
pub use quadtrees::bounds::*;

// Export geometry items
pub use geom::geometry::*;
pub use geom::euclidean;
pub use geom::euclidean::*;
pub use geom::spherical;
pub use geom::spherical::*;