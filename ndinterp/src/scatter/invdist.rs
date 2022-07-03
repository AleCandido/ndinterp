use super::{
    knn::{All, HNSW, KNN},
    utils::split_2d,
};
use crate::{interpolate::Interpolate, metric::Metric};

use ndarray::{Array1, Array2};

pub struct InvDistBase<Point, Finder>
where
    Point: Metric,
    Finder: KNN<Point = Point>,
{
    points: Vec<Point>,
    values: Vec<f64>,
    knn: Finder,
}

impl<Point, Finder> InvDistBase<Point, Finder>
where
    Point: Metric,
    Finder: KNN<Point = Point>,
{
    pub fn new(points: Vec<(Point, f64)>, knn: Finder) -> Self {
        let values = points.iter().map(|p| p.1).collect();

        Self {
            points: points.into_iter().map(|p| p.0).collect(),
            values,
            knn,
        }
    }
}

impl<Finder> InvDistBase<Array1<f64>, Finder>
where
    Finder: KNN<Point = Array1<f64>>,
{
    pub fn from_array(points: Array2<f64>, knn: Finder) -> Self {
        let (points, values) = split_2d(points);

        Self {
            points,
            values,
            knn,
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

        for nb_id in self.knn.neighbors(&(query)) {
            let dist = Point::distance(query, &self.points[nb_id]);
            let nb_value = self.values[nb_id];

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
