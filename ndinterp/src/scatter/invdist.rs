use super::{knn::KNN, metric::Metric};

pub struct InvDist<Point: Metric, Finder: KNN<Point>> {
    points: Vec<Point>,
    knn: Finder,
}

impl<Point: Metric, Finder: KNN<Point>> InvDist<Point, Finder> {
    pub fn new(points: Vec<Point>, knn: Finder) -> Self {
        Self { points, knn }
    }
}
