///! Implements cubic interpolation algorithms
use crate::grid::Grid;
pub use crate::interpolate::Interpolator;

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
///
///
#[derive(Debug)]
pub struct Cubic1d {
    pub grid: Grid,
}

impl Interpolator<f64> for Cubic1d {
    /// Use Cubic interpolation 1d to compute y(query)
    fn interpolate(&self, query: f64) -> f64 {
        let idx = self.grid.index_of(query);

        let dx = self.grid.input[idx + 1] - self.grid.input[idx];

        // Upper and lower bounds and derivatives
        let yu = self.grid.values[idx + 1];
        let yl = self.grid.values[idx];

        let dydxu = dx * self.grid.derivative_at(idx + 1);
        let dydxl = dx * self.grid.derivative_at(idx);

        let t = (query - self.grid.input[idx]) / dx;
        let t2 = t * t;
        let t3 = t2 * t;

        let p0 = yl * (2. * t3 - 3. * t2 + 1.);
        let p1 = yu * (-2. * t3 + 3. * t2);
        let m0 = dydxl * (t3 - 2. * t2 + t);
        let m1 = dydxu * (t3 - t2);

        return p0 + p1 + m0 + m1;
    }
}
