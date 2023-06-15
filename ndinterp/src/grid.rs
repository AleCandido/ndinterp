//! Interpolation algorithms for gridded base points.
//!
//! A Grid `struct` takes as input a D-dimensional tensor with the values of the variables:
//! (\vec{x1}, \vec{x2}, \vec{x3}...) and a 1-dimensional vector for the value at every point in x
//! \vec{y}
//!
//! The input arrays are always assumed to be sorted
//!
//! Several algorithms are provided to compute then the function
//!     y = f(x1, x2, x3...)
//!
use crate::interpolate::InterpolationError;
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
    // TODO: at the moment we are using here the derivatives that LHAPDF is using for the
    // interpolation in alpha_s, these are probably enough for this use case but not in general
    // - [ ] Implement a more robust form of the derivative
    // - [ ] Benchmark it against this one to study the impact in the performance of the code
    //

    /// Computes the "numerical derivative" of the values (`grid.values`) with respect to the
    /// input at position index as the ratio between the differences dy/dx computed as:
    ///     dy = y_{i} - y_{i-1}
    ///     dx = x_{i} - x_{x-1}
    pub fn derivative_at(&self, index: usize) -> f64 {
        let dx = self.input[index] - self.input[index - 1];
        let dy = self.values[index] - self.values[index - 1];
        return dy / dx;
    }

    /// Computes the numerical derivative of the values (`grid.values`) with respect to the input
    /// at position `i` as the average of the forward and backward differences, i.e.,
    ///
    /// Dx_{i} = \Delta x_{i} = x_{i} - x_{i-}
    /// y'_{i} = 1/2 * ( (y_{i+1}-y_{i})/Dx_{i+1} + (y_{i}-y_{i-1})/Dx_{i} )
    pub fn central_derivative_at(&self, index: usize) -> f64 {
        let dy_f = self.derivative_at(index + 1);
        let dy_b = self.derivative_at(index);
        return 0.5 * (dy_f + dy_b);
    }

    /// Find the index of the last value in the input such that input(idx) < query
    /// If the query is outside the grid returns an extrapolation error
    pub fn closest_below(&self, query: f64) -> Result<usize, InterpolationError> {
        if query > self.input[self.input.len() - 1] {
            Err(InterpolationError::ExtrapolationAbove(query))
        } else if query < self.input[0] {
            Err(InterpolationError::ExtrapolationBelow(query))
        } else {
            let u_idx = self.input.iter().position(|x| x > &query).unwrap();
            let idx = u_idx - 1;
            Ok(idx)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    fn gen_grid() -> Grid {
        let x = array![0., 1., 2., 3., 4.];
        let y = array![4., 3., 2., 1., 1.];
        let grid = Grid {
            input: x,
            values: y,
        };
        return grid;
    }

    #[test]
    fn check_derivative() {
        let grid = gen_grid();
        assert_eq!(grid.central_derivative_at(1), -1.);
        assert_eq!(grid.central_derivative_at(3), -0.5);
    }

    #[test]
    fn check_index_search() {
        let grid = gen_grid();
        assert_eq!(grid.closest_below(0.5).unwrap(), 0);
        assert_eq!(grid.closest_below(3.2).unwrap(), 3);
    }
}
