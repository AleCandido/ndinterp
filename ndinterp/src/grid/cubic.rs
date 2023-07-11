//! Implements cubic interpolation algorithms

use crate::grid::Grid;
use crate::interpolate::InterpolationError;
pub use crate::interpolate::Interpolator;
use ndarray::Ix1;

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
        let xgrid = &self.grid.xgrid;


        let dx = xgrid[0][idx + 1] - xgrid[0][idx];

        // Upper and lower bounds and derivatives
        let yu = self.grid.values[idx + 1];
        let yl = self.grid.values[idx];

        let dydxu = if idx == xgrid[0].len() - 2 {
            dx * self.grid.derivative_at(idx + 1)
        } else {
            dx * self.grid.central_derivative_at(idx + 1)
        };

        let dydxl = if idx == 0 {
            dx * self.grid.derivative_at(idx + 1)
        } else {
            dx * self.grid.central_derivative_at(idx)
        };

        let t = (query - xgrid[0][idx]) / dx;
        let t2 = t * t;
        let t3 = t2 * t;

        let p0 = yl * (2. * t3 - 3. * t2 + 1.);
        let p1 = yu * (-2. * t3 + 3. * t2);
        let m0 = dydxl * (t3 - 2. * t2 + t);
        let m1 = dydxu * (t3 - t2);

        Ok(p0 + p1 + m0 + m1)
    }
}
