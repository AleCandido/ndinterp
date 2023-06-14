///! Implements cubic interpolation algorithms
use crate::grid::Grid;
pub use crate::interpolate::Interpolator;

#[derive(Debug)]
pub struct Cubic1d {
    pub grid: Grid,
}

impl Interpolator<f64> for Cubic1d {
    fn interpolate(&self, query: f64) -> f64 {
        // find the index of the minimum
        // https://stackoverflow.com/questions/53903318/what-is-the-idiomatic-way-to-get-the-index-of-a-maximum-or-minimum-floating-poin
        let input_minus_val = self.grid.input.mapv(|a| (a - query).abs());

        let idx = input_minus_val
            .iter() // iterate over index
            .enumerate() // and create an enumerator iterator to which iterator::min_by will be applied
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, _)| i) // `min_by` returned an Option(i, float), we don't care about the float here
            .unwrap();

        let dxi = self.grid.input[idx + 1] - self.grid.input[idx];
        let tx = (query - self.grid.input[idx]) / dxi;

        // Upper and lower bounds and derivatives
        let yu = self.grid.values[idx + 1];
        let yl = self.grid.values[idx];

        let dydxu =
            dxi * 0.5 * (self.grid.derivative_at(idx + 2) + self.grid.derivative_at(idx + 1));
        let dydxl = dxi * 0.5 * (self.grid.derivative_at(idx) + self.grid.derivative_at(idx + 1));

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
