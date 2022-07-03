//! Utilities to find K-nearest neighbors.
//!
//! To be used as part of a generic scattered interpolation algorithm.
use crate::metric::Metric;

use petgraph::graph::UnGraph;

pub trait KNN {
    type Point;

    // Returns points identifiers
    fn neighbors(&self, query: &Self::Point) -> Vec<usize>;
}

pub struct All<'a, Point> {
    identifiers: Vec<usize>,
    points: Vec<&'a Point>,
}

impl<'a, Point> All<'a, Point> {
    pub fn new(points: Vec<(usize, &'a Point)>) -> Self {
        Self {
            identifiers: points.iter().map(|e| e.0).collect(),
            points: points.into_iter().map(|e| e.1).collect(),
        }
    }

    pub fn points(&self) -> &Vec<&Point> {
        &self.points
    }
}

impl<'a, Point> KNN for All<'a, Point> {
    type Point = Point;

    fn neighbors(&self, _: &Point) -> Vec<usize> {
        self.identifiers.clone()
    }
}

pub struct HNSW<'a, Point: Metric> {
    k: u32,
    graph: UnGraph<(usize, &'a Point), f64>,
}

impl<'a, Point: Metric> HNSW<'a, Point> {
    pub fn new(k: u32) -> Self {
        return HNSW {
            k,
            graph: UnGraph::<(usize, &Point), f64>::new_undirected(),
        };
    }

    pub fn k(&self) -> u32 {
        self.k
    }
}

impl<'a, Point: Metric + Clone> KNN for HNSW<'a, Point> {
    type Point = Point;

    fn neighbors(&self, _query: &Point) -> Vec<usize> {
        self.graph.raw_nodes().iter().map(|n| n.weight.0).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        let all = All::<f64>::new(vec![]);

        assert_eq!(all.neighbors(&10.).len(), 0);
    }
}
