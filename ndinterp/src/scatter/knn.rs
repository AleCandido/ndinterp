//! Utilities to find K-nearest neighbors.
//!
//! To be used as part of a generic scattered interpolation algorithm.
use crate::metric::Metric;

use petgraph::graph::UnGraph;

use std::rc::{Rc, Weak};

pub trait KNN {
    type Point;

    // Returns points identifiers
    fn neighbors(&self, query: &Self::Point) -> Vec<usize>;
}

pub struct All<Point> {
    identifiers: Vec<usize>,
    points: Vec<Weak<Point>>,
}

impl<Point> All<Point> {
    pub fn new(points: Vec<(usize, Rc<Point>)>) -> Self {
        Self {
            identifiers: points.iter().map(|e| e.0).collect(),
            points: points.into_iter().map(|e| Rc::downgrade(&e.1)).collect(),
        }
    }

    pub fn points(&self) -> &Vec<Weak<Point>> {
        &self.points
    }
}

impl<Point> KNN for All<Point> {
    type Point = Point;

    fn neighbors(&self, _: &Point) -> Vec<usize> {
        self.identifiers.clone()
    }
}

pub struct HNSW<Point: Metric> {
    k: u32,
    graph: UnGraph<(usize, Weak<Point>), f64>,
}

impl<Point: Metric> HNSW<Point> {
    pub fn new(k: u32) -> Self {
        HNSW {
            k,
            graph: UnGraph::<(usize, Weak<Point>), f64>::new_undirected(),
        }
    }

    pub fn k(&self) -> u32 {
        self.k
    }
}

impl<Point: Metric + Clone> KNN for HNSW<Point> {
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
