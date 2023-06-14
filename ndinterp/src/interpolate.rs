use std::iter::zip;

use crate::metric::Metric;

pub trait Interpolator<T> {
    fn interpolate(&self, query: T) -> f64;
}

/// ---- deal with the stuff below later ----
pub trait Interpolate {
    type Point: Metric;

    fn interpolate(&self, query: &Self::Point) -> f64;
}

pub struct Input<Point: Metric> {
    pub point: Point,
    pub value: f64,
}

impl<Point: Metric> From<(Point, f64)> for Input<Point> {
    fn from(item: (Point, f64)) -> Self {
        Self {
            point: item.0,
            value: item.1,
        }
    }
}

impl<Point: Metric> Input<Point> {
    pub fn stack(points: Vec<Point>, values: Vec<f64>) -> Vec<Self> {
        zip(points.into_iter(), values.into_iter())
            .map(|t| t.into())
            .collect()
    }
}
