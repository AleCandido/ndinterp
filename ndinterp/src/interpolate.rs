use crate::metric::Metric;

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
