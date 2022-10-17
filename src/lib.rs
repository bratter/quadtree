/*
 * Quadtree Package.
 * 
 * Multiple quadtree implementations for various geometries.
 * 
 * TODO: Add other failure conditions in find/knn e.g. at least haversine distance calc can fail, so dist_geom should be a result, and maybe other bbox functions also
 * TODO: Consumers need an easy time of sending radians into dist - perhaps just to_radians on Point is fine?
 * TODO: Distance sorted iterator
 * TODO: Retrieve as an iterator that doesn't error, just straight to None if out of bounds, or bbox fails (start by converting get_all_children to an iterator)
 * TODO: Document or test that retrieve.filter can give intersections or contains; convert retrieve to an iterator?
 * TODO: Write documentation for everything
 * TODO: Add more Haversine implementations for Spherical math
 * TODO: Force constraints on spherical coords?
 * TODO: Add clear and remove operations to the quadtree trait
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
mod error;

use geom::*;
use node::*;
use iter::*;

// Export the quadtree traits/constants, and implementations
pub use quadtrees::*;
pub use quadtrees::point::*;
pub use quadtrees::bounds::*;
pub use error::*;

// Export geometry items
pub use geom::geometry::*;
pub use geom::euclidean;
pub use geom::euclidean::*;
pub use geom::spherical;
pub use geom::spherical::*;