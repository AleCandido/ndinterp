//! Interpolation algorithms for a generic set of points.
//!
//! The more generic ones might apply even for generic metrics sets, without the need of a vector
//! space structure.
//! Nevertheless, they are all developed and optimize for the Euclidean n-dimenional spaces (if
//! you want to use for something, take care of differences, possibly not properly accounted for by
//! the implemented algorithms).

pub mod invdist;
pub mod knn;
pub mod utils;

pub use invdist::{InvDist, InvDistAll};
