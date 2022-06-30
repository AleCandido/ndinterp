//! Define concept of metric
use ndarray::prelude::*;
use ndarray_linalg::norm::Norm;

pub trait Metric {
    fn distance(a: &Self, b: &Self) -> f64;
}

impl<D: Dimension> Metric for Array<f64, D> {
    fn distance(a: &Self, b: &Self) -> f64 {
        (a - b).norm()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trivial_test() {
        let a = array![1., 2., 3.];

        assert_eq!(Array::distance(&a, &a), 0.);

        let b = array![1., 2., 1.];

        assert_eq!(Array::distance(&a, &b), 2.)
    }
}
