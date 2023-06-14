//! Interpolation algorithms for gridded base points.
//!
//! This module provides a struct Grid which takes as input a D-dimensional tensor
//! with the values of (\vec{x1}, \vec{x2}, \vec{x3}...) and a 1-dimensional vector for the input at every point x
//!
//! Several algorithms are provided to compute then the function
//!     y = f(x1, x2, x3...)
//!

use ndarray::Array1;

// Make public the families of interpolation algorithms implemented for grids
pub mod cubic;

#[derive(Debug)]
pub struct Grid {
    /// A grid is made of two (1-dimensional) sorted arrays.
    pub input: Array1<f64>,
    pub values: Array1<f64>,
}

impl Grid {
    pub fn derivative_at(&self, index: usize) -> f64 {
        // Computes the derivative of values with respect to the input at the position i = index
        // y = (val[i] - val[i-1])/(input[i] - input[i-1])
        let h = self.input[index] - self.input[index - 1];
        let dy = self.values[index] - self.values[index - 1];
        return dy / h;
    }
}
