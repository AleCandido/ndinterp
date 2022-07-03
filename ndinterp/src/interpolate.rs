pub trait Interpolate {
    type Point;

    fn interpolate(&self, query: &Self::Point) -> f64;
}
