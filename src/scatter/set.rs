//! Hold and manage a set of base points for ungridded interpolation.

pub struct Set<Point> {
    points: Vec<Point>,
    dist: fn(Point, Point) -> f64,
}

impl<Point> Set<Point> {
    pub fn new(points: Vec<Point>, dist: fn(Point, Point) -> f64) -> Self {
        Set { points, dist }
    }

    pub fn distance(&self, a: Point, b: Point) -> f64 {
        (self.dist)(a, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_test() {
        let s = Set {
            points: vec![0., 1., 2.],
            dist: |a: f64, b| (a - b).abs(),
        };

        assert_eq!(s.distance(s.points[0], s.points[1]), 1.)
    }
}
