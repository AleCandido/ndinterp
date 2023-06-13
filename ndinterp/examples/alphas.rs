/// In this example we utilize the cubic_1d interpolation implemented for the Grid struct to
/// perform alpha_s(q) interpolation.
/// The example values in this file have been obtained with LHAPDF6 for NNPDF40_nnlo_as_01180
use ndarray::array;
use ndinterp::grid::{Grid, Interpolation};

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
        input: logq2,
        values: alpha_s_vals,
    };

    let example_q: Vec<f64> = vec![1.8, 2.6, 3.4, 4.1];
    let lhapdf_res = vec![0.31652747, 0.26841305, 0.24201896, 0.22660515];
    for (i, qval) in example_q.iter().enumerate() {
        let q2val = qval.powf(2.0);
        let lq2 = f64::ln(q2val);
        let vpol = grid.cubic_1d(lq2);

        println!(
            "Interpolated value: alpha_s({}) = {:.4} (lhapdf = {:.4})",
            qval, vpol, lhapdf_res[i]
        );
    }
}
