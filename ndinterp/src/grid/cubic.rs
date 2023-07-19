//! Implements cubic interpolation algorithms
//!
//! These are the algorithms used by the LHAPDF library for `alpha_s` and pdf(x, q)
//!

use crate::grid::{DimensionHelper, Grid, GridSlice, ToDimension};
use crate::interpolate::InterpolationError;
pub use crate::interpolate::Interpolator;
use itertools::izip;
use ndarray::Axis;

/// Cubic interpolation
#[derive(Debug)]
pub struct Cubic<const D: usize>
where
    DimensionHelper<D>: ToDimension,
{
    /// The grid object contains all necessary information to perform the interpolation
    pub grid: Grid<D>,
}

///
/// In 1D the interpolation is such that given t in an interval [t0, tn] such that
///     p0 = y(t0)  ; p1 = y(tn)
///     m0 = y'(t0) ; m1 = y'(tn)
///     dx = ti+1 - ti
/// returns the value of y(ti) (with 0 < i < n):
///         y(t) = h00(t)*p0 + h10(t)*m0*dx + h01(t)*p1 + h11(t)*m1*dx
/// with hij the Hermite basis functions
fn cubic_interpolation_1d(t: f64, yl: f64, yu: f64, dydxl: f64, dydxu: f64) -> f64 {
    let t2 = t * t;
    let t3 = t2 * t;

    let p0 = yl * (2. * t3 - 3. * t2 + 1.);
    let p1 = yu * (-2. * t3 + 3. * t2);
    let m0 = dydxl * (t3 - 2. * t2 + t);
    let m1 = dydxu * (t3 - t2);

    p0 + p1 + m0 + m1
}

impl<'a> GridSlice<'a> {
    /// Implements utilities for a GridSlice that can be used by cubic interpolation Nd
    /// Takes as input the value being queried and its index within the given slice

    /// Perform 1d cubic interpolation such that f(x) = y
    fn cubic_interpolate_1d(&'a self, query: f64, idx: usize) -> f64 {
        // grid slice utilities are expected to be called multipled times for the same
        // query and so it is convient to pass idx from the outside to avoid expensive searches
        let dx = self.x[idx + 1] - self.x[idx];

        // Upper and lower bounds and derivatives
        let yu = self.y[idx + 1];
        let yl = self.y[idx];

        let dydxu = if idx == self.x.len() - 2 {
            dx * self.derivative_at(idx + 1)
        } else {
            dx * self.central_derivative_at(idx + 1)
        };

        let dydxl = if idx == 0 {
            dx * self.derivative_at(idx + 1)
        } else {
            dx * self.central_derivative_at(idx)
        };

        let t = (query - self.x[idx]) / dx;

        cubic_interpolation_1d(t, yl, yu, dydxl, dydxu)
    }
}

impl Interpolator<f64> for Cubic<1> {
    /// Use Cubic interpolation 1d to compute y(query)
    /// The interpolation uses the two nearest neighbours and their derivatives computed as an
    /// average of the differences above and below.
    ///
    /// Special cases are considered when the interpolation occurs between the first (last) two
    /// bins, where the derivative would involve points outside the grids.
    ///
    /// Two special are considered, when the interpolation occurs between the first (last) two
    /// bins, the derivative at the boundary is approximated by the forward (backward) difference
    fn interpolate(&self, query: f64) -> Result<f64, InterpolationError> {
        let raw_idx = self.grid.closest_below(&[query])?;
        let idx = raw_idx[0];

        let grid_sl = GridSlice {
            x: &self.grid.xgrid[0],
            y: self.grid.values.view(),
        };

        Ok(grid_sl.cubic_interpolate_1d(query, idx))
    }
}

impl Interpolator<&[f64]> for Cubic<2> {
    /// Use Cubic interpolation 2d to compute y(x1, x2)
    fn interpolate(&self, query: &[f64]) -> Result<f64, InterpolationError> {
        let raw_idx = self.grid.closest_below(query)?;

        let x1 = query[0]; // x1 correspond to x in PDF interpolation
        let x2 = query[1];
        let id_x1 = raw_idx[0];
        let id_x2 = raw_idx[1];
        let x1_grid = &self.grid.xgrid[0];
        let x2_grid = &self.grid.xgrid[1];
        let yvals = &self.grid.values;

        // First interpolate in x1 by taken the nodes around the x2 index
        // Create slices in x1 for values in x2 at (i+2, i+1, <query>, i, i-1)
        let mut vs = [0.0; 4];
        for (v, i) in izip!(&mut vs, (id_x2 - 1)..(id_x2 + 3)) {
            *v = GridSlice {
                x: x1_grid,
                y: yvals.index_axis(Axis(1), i),
            }
            .cubic_interpolate_1d(x1, id_x1);
        }

        // Now perform the interpolation in x2
        let dx2_0 = x2_grid[id_x2] - x2_grid[id_x2 - 1];
        let dx2_1 = x2_grid[id_x2 + 1] - x2_grid[id_x2];
        let dx2_2 = x2_grid[id_x2 + 2] - x2_grid[id_x2 + 1];

        let lower_derivative = 0.5 * ((vs[2] - vs[1]) + (vs[1] - vs[0]) * dx2_1 / dx2_0);
        let upper_derivative = 0.5 * ((vs[2] - vs[1]) + (vs[3] - vs[2]) * dx2_1 / dx2_2);

        let t = (x2 - x2_grid[id_x2]) / dx2_1;

        Ok(cubic_interpolation_1d(
            t,
            vs[1],
            vs[2],
            lower_derivative,
            upper_derivative,
        ))
    }
}
