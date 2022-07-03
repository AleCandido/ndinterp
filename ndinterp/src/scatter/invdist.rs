use super::{knn::KNN, metric::Metric};
use crate::interpolate::Interpolate;

use hashbrown::HashMap;

use std::{cmp::Eq, hash::Hash};

pub struct InvDist<Point, Finder>
where
    Point: Metric,
    Finder: KNN<Point>,
{
    values: HashMap<Point, f64>,
    knn: Finder,
}

impl<Point: Metric, Finder: KNN<Point>> InvDist<Point, Finder> {
    pub fn new(values: HashMap<Point, f64>, knn: Finder) -> Self {
        Self { values, knn }
    }
}

impl<Point, Finder> Interpolate for InvDist<Point, Finder>
where
    Point: Metric + Hash + Eq,
    Finder: KNN<Point>,
{
    type Point = Point;

    fn interpolate(&self, query: &Point) -> f64 {
        let neighbors = self.knn.neighbors(query);

        let mut value = 0.;
        let mut norm = 0.;

        for n in neighbors {
            let dist = Point::distance(query, &n);
            let nvalue = self.values.get(&n).unwrap().clone();

            // In case of distance too close, early return the exact value
            if dist < 1e-10 {
                return nvalue;
            }

            value += nvalue / dist;
            norm += 1. / dist;
        }

        value / norm
    }
}
