/// In this example we utilize the cubic_1d interpolation implemented for the Grid struct to
/// perform alpha_s(q) interpolation.
/// The example values in this file have been obtained with LHAPDF6 for NNPDF40_nnlo_as_01180
///
/// The numbers tested cover the following situations:
/// 1. Interpolation between the first two bins (1.7)
/// 2. Interpolation in the region between the second and next-to-last bin
/// 3. Interpolation between the last two bins (4.5)
/// 4. Extrapolation above and below (results in an error!)
use ndarray::array;

use ndinterp::grid::cubic::{Cubic1d, Interpolator};
use ndinterp::grid::Grid;
use ndinterp::interpolate::InterpolationError;

fn main() {
    println!("Testing 1d cubic interpolation: alpha_s(Q)");

    // Copy the values of q < 4.92 and corresponding alpha_s for NNPDF40
    let q2s = array![
        2.7225,
        3.19493746,
        3.774881,
        4.49174997,
        5.38430257,
        6.50400153,
        7.91973571,
        9.72449465,
        12.04490818,
        15.05498278,
        18.99610035,
        24.2064
    ];
    // LHAPDF does the interpolation in alphas in a logarithmic manner
    let logq2 = q2s.mapv(f64::ln);

    let alpha_s_vals = array![
        0.33074891, 0.3176246, 0.30507081, 0.29305875, 0.28156114, 0.27055221, 0.26000761,
        0.24990438, 0.24022086, 0.23093662, 0.22203241, 0.21377883
    ];

    let grid = Grid {
        input: vec![logq2.to_vec()],
        values: alpha_s_vals,
    };

    let cubic_interpolator = Cubic1d { grid };

    let example_q: Vec<f64> = vec![1.7, 1.8, 2.6, 3.4, 4.1, 4.5, 10.0, 1.0];
    let lhapdf_res = vec![
        0.32580476, 0.31652747, 0.26841305, 0.24201896, 0.22660515, 0.21978229, 0.0, 1.0,
    ];
    for (i, qval) in example_q.iter().enumerate() {
        let q2val = qval.powf(2.0);
        let lq2 = f64::ln(q2val);
        match cubic_interpolator.interpolate(lq2) {
            Ok(vpol) => println!(
                "Interpolated value: alpha_s({}) = {:.4} (lhapdf = {:.4})",
                qval, vpol, lhapdf_res[i]
            ),
            Err(error) => {
                if let InterpolationError::ExtrapolationAbove(_) = error {
                    eprintln!("Extrapolating for Q>Qmax: {}", error);
                } else if let InterpolationError::ExtrapolationBelow(_) = error {
                    eprintln!("Extrapolating for Q<Qmin: {}", error);
                }
            }
        }
    }
}
