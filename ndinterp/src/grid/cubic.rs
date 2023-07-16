//! Implements cubic interpolation algorithms

use crate::grid::{Grid, GridSlice};
use crate::interpolate::InterpolationError;
pub use crate::interpolate::Interpolator;
use ndarray::{Axis, Ix1, Ix2};

/// Cubic interpolation in 1D
///
/// Note: this is the interpolation algorithm used by the LHAPDF library for `alpha_s`, in LHAPDF
/// the interpolation variable is `log(q^2)`
///
/// Given t in an interval [t0, tn] such that
///     p0 = y(t0)  ; p1 = y(tn)
///     m0 = y'(t0) ; m1 = y'(tn)
///     dx = ti+1 - ti
/// returns the value of y(ti) (with 0 < i < n):
///         y(t) = h00(t)*p0 + h10(t)*m0*dx + h01(t)*p1 + h11(t)*m1*dx
/// with hij the Hermite basis functions
#[derive(Debug)]
pub struct Cubic1d {
    /// The grid object contains all necessary information to perform the interpolation
    pub grid: Grid<Ix1>,
}

/// Cubic interpolation in 2D
#[derive(Debug)]
pub struct Cubic2d {
    /// The grid object contains all necessary information to perform the interpolation
    pub grid: Grid<Ix2>,
}

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
    fn cubic_interpolate_1d(&'a self, query: f64, idx: usize) -> Result<f64, InterpolationError> {
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

        Ok(cubic_interpolation_1d(t, yl, yu, dydxl, dydxu))
    }
}

impl Interpolator<f64> for Cubic1d {
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
        let raw_idx = self.grid.closest_below::<1>(&[query])?;
        let idx = raw_idx[0];

        let grid_sl = GridSlice {
            x: &self.grid.xgrid[0],
            y: self.grid.values.view(),
        };

        grid_sl.cubic_interpolate_1d(query, idx)
    }
}

impl Interpolator<&[f64]> for Cubic2d {
    /// Use Cubic interpolation 2d to compute y(x1, x2)
    fn interpolate(&self, query: &[f64]) -> Result<f64, InterpolationError> {
        let raw_idx = self.grid.closest_below::<2>(query)?;

        let x1 = query[0]; // x1 correspond to x in PDF interpolation
        let x2 = query[1];
        let id_x1 = raw_idx[0];
        let id_x2 = raw_idx[1];
        let x1_grid = &self.grid.xgrid[0];
        let x2_grid = &self.grid.xgrid[1];
        let yvals = &self.grid.values;

        // First interpolate in x1 by taken the nodes around the x2 index
        // Create slices in x1 for values in x2 at (i+2, i+1, <query>, i, i-1)
        let slice_x1_m2 = GridSlice {
            x: x1_grid,
            y: yvals.index_axis(Axis(1), id_x2 - 1),
        };
        let slice_x1_m1 = GridSlice {
            x: x1_grid,
            y: yvals.index_axis(Axis(1), id_x2),
        };
        let slice_x1_p1 = GridSlice {
            x: x1_grid,
            y: yvals.index_axis(Axis(1), id_x2 + 1),
        };
        let slice_x1_p2 = GridSlice {
            x: x1_grid,
            y: yvals.index_axis(Axis(1), id_x2 + 2),
        };

        let vm2 = slice_x1_m2.cubic_interpolate_1d(x1, id_x1)?;
        let vm1 = slice_x1_m1.cubic_interpolate_1d(x1, id_x1)?;
        let vp1 = slice_x1_p1.cubic_interpolate_1d(x1, id_x1)?;
        let vp2 = slice_x1_p2.cubic_interpolate_1d(x1, id_x1)?;

        // Now perform the interpolation in x2
        let dx2_0 = x2_grid[id_x2] - x2_grid[id_x2 - 1];
        let dx2_1 = x2_grid[id_x2 + 1] - x2_grid[id_x2];
        let dx2_2 = x2_grid[id_x2 + 2] - x2_grid[id_x2 + 1];

        let lower_derivative = 0.5 * ((vp1 - vm1) + (vm1 - vm2) * dx2_1 / dx2_0);
        let upper_derivative = 0.5 * ((vp1 - vm1) + (vp2 - vp1) * dx2_1 / dx2_2);

        let t = (x2 - x2_grid[id_x2]) / dx2_1;

        Ok(cubic_interpolation_1d(
            t,
            vm1,
            vp1,
            lower_derivative,
            upper_derivative,
        ))
    }
}
