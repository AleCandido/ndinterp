//! This module implements interpolation rutines
use thiserror::Error;

/// Errors encountered during interpolation
#[derive(Debug, Error)]
pub enum InterpolationError {
    /// Raised when the queried value is above the maximum
    #[error("The value queried ({0}) is above the maximum")]
    ExtrapolationAbove(f64),

    /// Raised when the queried value is below the minimum
    #[error("The value queried ({0}) is below the minimum")]
    ExtrapolationBelow(f64),
}

/// Methods which all interpolator must implement
pub trait Interpolator<T> {
    /// Produce the result of the inteprolation given a (nd) point 'query'
    fn interpolate(&self, query: T) -> Result<f64, InterpolationError>;
}

///// ---- deal with the stuff below later ----
//pub trait Interpolate {
//    type Point: Metric;
//
//    fn interpolate(&self, query: &Self::Point) -> f64;
//}
//
//pub struct Input<Point: Metric> {
//    pub point: Point,
//    pub value: f64,
//}
//
//impl<Point: Metric> From<(Point, f64)> for Input<Point> {
//    fn from(item: (Point, f64)) -> Self {
//        Self {
//            point: item.0,
//            value: item.1,
//        }
//    }
//}
//
//impl<Point: Metric> Input<Point> {
//    pub fn stack(points: Vec<Point>, values: Vec<f64>) -> Vec<Self> {
//        zip(points.into_iter(), values.into_iter())
//            .map(|t| t.into())
//            .collect()
//    }
//}
