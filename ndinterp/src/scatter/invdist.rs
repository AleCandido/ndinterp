use super::{
    commons::Commons,
    knn::{All, HNSW, KNN},
};
use crate::{
    interpolate::{Input, Interpolate},
    metric::Metric,
};

use ndarray::{Array1, Array2};

pub struct InvDistBase<Point, Finder>
where
    Point: Metric,
    Finder: KNN<Point = Point>,
{
    commons: Commons<Point, Finder>,
}

impl<Point, Finder> InvDistBase<Point, Finder>
where
    Point: Metric,
    Finder: KNN<Point = Point>,
{
    pub fn new(inputs: Vec<Input<Point>>) -> Self {
        Self {
            commons: Commons::new(inputs),
        }
    }

    pub fn set_finder(&mut self, finder: Finder) {
        self.commons.set_finder(finder)
    }
}

impl<Finder> From<Array2<f64>> for InvDistBase<Array1<f64>, Finder>
where
    Finder: KNN<Point = Array1<f64>>,
{
    fn from(inputs: Array2<f64>) -> Self {
        Self {
            commons: inputs.into(),
        }
    }
}

impl<Point, Finder> Interpolate for InvDistBase<Point, Finder>
where
    Point: Metric,
    Finder: KNN<Point = Point>,
{
    type Point = Point;

    fn interpolate(&self, query: &Self::Point) -> f64 {
        let mut value = 0.;
        let mut norm = 0.;

        for nb_id in self
            .commons
            .finder
            .as_ref()
            .expect("Finder not set.")
            .neighbors(query)
        {
            let dist = Point::distance(query, &self.commons.points[nb_id]);
            let nb_value = self.commons.values[nb_id];

            // In case of distance too close, early return the exact value
            if dist < 1e-10 {
                return nb_value;
            }

            value += nb_value / dist;
            norm += 1. / dist;
        }

        value / norm
    }
}

pub type InvDistAll = InvDistBase<Array1<f64>, All<Array1<f64>>>;
pub type InvDist = InvDistBase<Array1<f64>, HNSW<Array1<f64>>>;
