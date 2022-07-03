use super::knn::{All, HNSW, KNN};
use crate::{interpolate::Interpolate, metric::Metric};

use hashbrown::HashMap;
use ndarray::Array1;

use std::{cmp::Eq, hash::Hash};

pub struct InvDistBase<Point, Finder>
where
    Point: Metric,
    Finder: KNN<Point = Point>,
{
    values: HashMap<Point, f64>,
    knn: Finder,
}

impl<Point, Finder> InvDistBase<Point, Finder>
where
    Point: Metric,
    Finder: KNN<Point = Point>,
{
    pub fn new(values: HashMap<Point, f64>, knn: Finder) -> Self {
        Self { values, knn }
    }
}

impl<Point, Finder> Interpolate for InvDistBase<Point, Finder>
where
    Point: Metric + Hash + Eq,
    Finder: KNN<Point = Point>,
{
    type Point = Point;

    fn interpolate(&self, query: &Self::Point) -> f64 {
        let mut value = 0.;
        let mut norm = 0.;

        for n in self.knn.neighbors(&(query)) {
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

pub type InvDistAll = InvDistBase<Array1<f64>, All<Array1<f64>>>;
pub type InvDist = InvDistBase<Array1<f64>, HNSW<Array1<f64>>>;
