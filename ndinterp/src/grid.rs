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
use itertools::izip;
use ndarray::{Array, ArrayView1, Dimension};

// Make public the families of interpolation algorithms implemented for grids
pub mod cubic;

/// A grid is made of two components:
///     A d-dimensional vector of 1-dimensional sorted vectors for the input points
///     A d-dimensional array for the grid values of
#[derive(Debug)]
pub struct Grid<D: Dimension> {
    /// Arrays with the input vectors (x_i)
    pub xgrid: Vec<Vec<f64>>,

    /// Output points
    pub values: Array<f64, D>,
}

/// A grid slice is always 1-Dimensional
/// and it is made of the x and y values such that f(x) = y
#[derive(Debug)]
pub struct GridSlice<'a> {
    /// A reference to one of the input vectors of the grid
    pub x: &'a Vec<f64>,
    /// A view of the slice of values corresponding to x
    pub y: ArrayView1<'a, f64>,
}

impl<'a> GridSlice<'a> {
    // TODO: at the moment we are using here the derivatives that LHAPDF is using for the
    // interpolation in alpha_s, these are probably enough for this use case but not in general
    // - [ ] Implement a more robust form of the derivative
    // - [ ] Benchmark it against this one to study the impact in the performance of the code
    //

    /// Computes the "numerical derivative" of the values (`grid.values`) with respect to the
    /// input at position index as the ratio between the differences dy/dx computed as:
    ///     dy = y_{i} - y_{i-1}
    ///     dx = x_{i} - x_{x-1}
    pub fn derivative_at(&'a self, index: usize) -> f64 {
        let dx = self.x[index] - self.x[index - 1];
        let dy = self.y[index] - self.y[index - 1];
        dy / dx
    }

    /// Computes the numerical derivative of the values (`grid.values`) with respect to the input
    /// at position `i` as the average of the forward and backward differences, i.e.,
    ///
    /// Dx_{i} = \Delta x_{i} = x_{i} - x_{i-}
    /// y'_{i} = 1/2 * ( (y_{i+1}-y_{i})/Dx_{i+1} + (y_{i}-y_{i-1})/Dx_{i} )
    pub fn central_derivative_at(&'a self, index: usize) -> f64 {
        let dy_f = self.derivative_at(index + 1);
        let dy_b = self.derivative_at(index);
        0.5 * (dy_f + dy_b)
    }
}

impl<D: Dimension> Grid<D> {
    /// Find the index of the last value in the input xgrid such that xgrid(idx) < query
    /// If the query is outside the grid returns an extrapolation error
    pub fn closest_below<const N: usize>(
        &self,
        input_query: &[f64],
    ) -> Result<[usize; N], InterpolationError> {
        let mut ret = [0; N];

        for (r, &query, igrid) in izip!(&mut ret, input_query, &self.xgrid) {
            if query > *igrid.last().unwrap() {
                return Err(InterpolationError::ExtrapolationAbove(query));
            } else if query < igrid[0] {
                return Err(InterpolationError::ExtrapolationBelow(query));
            }

            let u_idx = igrid.partition_point(|&x| x < query);
            *r = u_idx - 1;
        }
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::{array, Ix1};

    fn gen_grid() -> Grid<Ix1> {
        let x = vec![vec![0., 1., 2., 3., 4.]];
        let y = array![4., 3., 2., 1., 1.];

        Grid {
            xgrid: x,
            values: y,
        }
    }

    #[test]
    fn check_derivative() {
        let grid = gen_grid();
        let grid_slice = GridSlice {
            x: &grid.xgrid[0],
            y: grid.values.view(),
        };
        assert_eq!(grid_slice.central_derivative_at(1), -1.);
        assert_eq!(grid_slice.central_derivative_at(3), -0.5);
    }

    #[test]
    fn check_index_search() {
        let grid = gen_grid();
        assert_eq!(grid.closest_below::<1>(&[0.5]).unwrap()[0], 0);
        assert_eq!(grid.closest_below::<1>(&[3.2]).unwrap()[0], 3);
    }
}
