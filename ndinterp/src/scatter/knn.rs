//! Utilities to find K-nearest neighbors.
//!
//! To be used as part of a generic scattered interpolation algorithm.
use crate::metric::Metric;

use petgraph::graph::UnGraph;

pub trait KNN {
    type Point;

    fn neighbors(&self, query: &Self::Point) -> Vec<Self::Point>;
}

pub struct All<Point: Clone> {
    points: Vec<Point>,
}

impl<Point: Clone> All<Point> {
    pub fn new(points: Vec<Point>) -> Self {
        Self { points }
    }
}

impl<Point: Clone> KNN for All<Point> {
    type Point = Point;

    fn neighbors(&self, _: &Point) -> Vec<Point> {
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

impl<Point: Metric + Clone> KNN for HNSW<Point> {
    type Point = Point;

    fn neighbors(&self, _query: &Point) -> Vec<Point> {
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
