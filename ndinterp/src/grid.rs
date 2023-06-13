//! Interpolation algorithms for gridded base points.
//!
//! This module provides a struct Grid which takes as input a D-dimensional tensor
//! with the values of (\vec{x1}, \vec{x2}, \vec{x3}...) and a 1-dimensional vector for the input at every point x
//!
//! Several algorithms are provided to compute then the function
//!     y = f(x1, x2, x3...)
//!

use ndarray::Array1;

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

pub trait Interpolation {
    fn cubic_1d(&self, val: f64) -> f64;
}

impl Interpolation for Grid {
    fn cubic_1d(&self, xval: f64) -> f64 {
        // find the index of the minimum
        // https://stackoverflow.com/questions/53903318/what-is-the-idiomatic-way-to-get-the-index-of-a-maximum-or-minimum-floating-poin
        let input_minus_val = self.input.mapv(|a| (a - xval).abs());

        let idx = input_minus_val
            .iter() // iterate over index
            .enumerate() // and create an enumerator iterator to which iterator::min_by will be applied
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, _)| i) // `min_by` returned an Option(i, float), we don't care about the float here
            .unwrap();

        let dxi = self.input[idx + 1] - self.input[idx];
        let tx = (xval - self.input[idx]) / dxi;

        // Upper and lower bounds and derivatives
        let yu = self.values[idx + 1];
        let yl = self.values[idx];

        let dydxu = dxi * 0.5 * (self.derivative_at(idx + 2) + self.derivative_at(idx + 1));
        let dydxl = dxi * 0.5 * (self.derivative_at(idx) + self.derivative_at(idx + 1));

        // Implementation of the cubic interpolation as seen in LHAPDF
        let t2 = tx * tx;
        let t3 = t2 * tx;

        let p0 = yl * (2. * t3 - 3. * t2 + 1.);
        let p1 = yu * (-2. * t3 + 3. * t2);
        let m0 = dydxl * (t3 - 2. * t2 + tx);
        let m1 = dydxu * (t3 - t2);

        return p0 + p1 + m0 + m1;
    }
}
