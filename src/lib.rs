/*
 * Quadtree Package.
 *
 * Multiple quadtree implementations for various geometries.
 *
 * TODO: Add clear and remove operations to the quadtree trait
 * TODO: Add more Haversine implementations for Spherical math
 * TODO: Build an integer-with-power-2-bounds version?
 * TODO: Should nodes and children be private on the node structs?
 * TODO: Force constraints on spherical coords?
 * TODO: Make a PR for the geo crate to add extra euclidean and haversine distance measures for Rect
 *       PR also should include fixing the TODO in https://docs.rs/geo/latest/src/geo/algorithm/contains/rect.rs.html#30-42
 *       This is wrong and should probably use > not >=
 */

mod error;
mod geom;
mod iter;
mod node;
mod quadtrees;

use geom::*;
use iter::*;
use node::*;

// Export the quadtree traits/constants, and implementations
pub use error::*;
pub use quadtrees::bounds::*;
pub use quadtrees::point::*;
pub use quadtrees::*;

// Export geometry items
pub use geom::euclidean;
pub use geom::euclidean::*;
pub use geom::geometry::*;
pub use geom::spherical;
pub use geom::spherical::to_radians::*;
pub use geom::spherical::*;
