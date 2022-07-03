use ndarray::{s, Array1, Array2};

pub fn split_2d(points: Array2<f64>) -> (Vec<Array1<f64>>, Vec<f64>) {
    let values = points.outer_iter().map(|ar| ar[ar.len() - 1]).collect();
    let points = points
        .slice(s![.., ..-1])
        .outer_iter()
        .map(|ar| ar.to_owned())
        .collect();

    (points, values)
}
