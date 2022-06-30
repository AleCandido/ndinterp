//! Utilities to find K-nearest neighbors.
//!
//! To be used as part of a generic scattered interpolation algorithm.

use super::metric::Metric;

pub struct KNN<Point: Metric> {
    edges: Vec<(Point, Point)>,
}

impl<Point: Metric> KNN<Point> {
    pub fn new(set: Vec<Point>) -> Self {
        return KNN { edges: vec![] };
    }
}
