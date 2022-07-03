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
    finder: Option<Finder>,
}

impl<Point, Finder> InvDistBase<Point, Finder>
where
    Point: Metric,
    Finder: KNN<Point = Point>,
{
    pub fn new(points: Vec<(Point, f64)>) -> Self {
        let values = points.iter().map(|p| p.1).collect();

        Self {
            points: points.into_iter().map(|p| p.0).collect(),
            values,
            finder: None,
        }
    }

    pub fn set_finder(&mut self, finder: Finder) {
        self.finder = Some(finder);
    }
}

impl<Finder> InvDistBase<Array1<f64>, Finder>
where
    Finder: KNN<Point = Array1<f64>>,
{
    pub fn from_array(points: Array2<f64>) -> Self {
        let (points, values) = split_2d(points);

        Self {
            points,
            values,
            finder: None,
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
            .finder
            .as_ref()
            .expect("Finder not set.")
            .neighbors(&(query))
        {
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

pub type InvDistAll<'a> = InvDistBase<Array1<f64>, All<'a, Array1<f64>>>;
pub type InvDist<'a> = InvDistBase<Array1<f64>, HNSW<'a, Array1<f64>>>;

impl<'a> InvDistBase<Array1<f64>, All<'a, Array1<f64>>> {
    fn from_points(points: Vec<(Array1<f64>, f64)>) -> Self {
        let mut inv_dist = Self::new(points);
        let all = All::<Array1<f64>>::new(
            inv_dist
                .points
                .iter()
                .enumerate()
                .collect::<Vec<(usize, &Array1<f64>)>>(),
        );

        inv_dist.set_finder(all);

        inv_dist
    }
}
