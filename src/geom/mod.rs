use geo::Rect;

// Module declarations
pub mod euclidean;
pub mod spherical;

// TODO: Document better/consider: User test points must implement one or both of DistEuclidean or DistSpherical
//       But would encourage just using geotypes for this where implementations come pre-made if the Datum is also a geotype
//       There might also be some way to impl for Derefs - see the section in rect.rs and try it out

/// Trait implemented by the geometry wrapper types which provides polymorphism
/// for distance calculations. The implementors should generally forward the
/// call to an appropriate underlying distance calculation, for example that
/// provided by the `EuclideanDistance` trait.
/// 
/// This trait should not need to be used outside the crate, as the only place
/// it is necessary is in the wrapper types. However any consumer can implement
/// custom distance calcs on any type they wish, so is still part of the public
/// API.
pub trait Distance<D> {
    /// Calculate the distance between a datum `D` and the test type
    /// implementing this trait.
    fn dist_datum(&self, datum: &D) -> f64;

    /// Calculate the distance between a geo::Rect` and the test type
    /// implementing this trait.
    fn dist_bbox(&self, bbox: &Rect) -> f64;
}