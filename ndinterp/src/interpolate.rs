use crate::metric::Metric;

pub trait Interpolate {
    type Point: Metric;

    fn interpolate(&self, query: &Self::Point) -> f64;
}
