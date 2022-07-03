//! Utilities to find K-nearest neighbors.
//!
//! To be used as part of a generic scattered interpolation algorithm.
use super::metric::Metric;

use petgraph::graph::UnGraph;

pub trait KNN<Point> {
    fn neighbors(&self, x: Point) -> Vec<Point>;
}

pub struct All<Point: Clone> {
    points: Vec<Point>,
}

impl<Point: Clone> All<Point> {
    pub fn new(points: Vec<Point>) -> Self {
        Self { points }
    }
}

impl<Point: Clone> KNN<Point> for All<Point> {
    fn neighbors(&self, _: Point) -> Vec<Point> {
        self.points.clone()
    }
}

pub struct HNSW<Point: Metric> {
    k: u32,
    graph: UnGraph<Point, f64>,
}

impl<Point: Metric> HNSW<Point> {
    pub fn new(k: u32) -> Self {
        return HNSW {
            k,
            graph: UnGraph::<Point, f64>::new_undirected(),
        };
    }

    pub fn k(&self) -> u32 {
        self.k
    }
}

impl<Point: Metric + Clone> KNN<Point> for HNSW<Point> {
    fn neighbors(&self, _x: Point) -> Vec<Point> {
        self.graph
            .raw_nodes()
            .iter()
            .map(|n| n.weight.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        let all = All::<f64>::new(vec![]);

        assert_eq!(all.neighbors(10.).len(), 0);
    }
}
