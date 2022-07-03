use ndarray::Array2;
use numpy::{PyReadonlyArray1, PyReadonlyArray2};
use pyo3::prelude::*;

use ndinterp::scatter::{self, knn};

#[pyclass]
#[repr(transparent)]
pub struct InvDistAll {
    pub(crate) interpolator: scatter::InvDistAll,
}

#[pymethods]
impl InvDistAll {
    #[new]
    pub fn new(points: Vec<(PyReadonlyArray1<f64>, f64)>) -> Self {
        Self {
            interpolator: scatter::InvDistAll::new(points),
        }
    }
}
