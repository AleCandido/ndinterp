//! Utilities to find K-nearest neighbors.
//!
//! To be used as part of a generic scattered interpolation algorithm.

use super::set::Set;

pub struct KNN<Point> {
    set: Set<Point>,
    edges: Vec<(Point, Point)>,
}

impl<Point> KNN<Point> {
    fn new(set: Set<Point>) -> Self {
        return KNN { set, edges: vec![] };
    }
}
