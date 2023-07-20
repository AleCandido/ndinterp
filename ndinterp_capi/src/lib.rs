//! C interface for ndinterp
#![warn(clippy::all, clippy::cargo)]
#![warn(missing_docs)]

use core::slice;

use ndarray::ArrayView1;
use ndinterp::grid;
use ndinterp::interpolate::Interpolator;

/// cubic1d inteprolator
pub struct Cubic1d;

/// Creates a cubic1d interpolator given the nodes
/// and the values of the function in said nodes
///
/// # Safety
///
/// This function is only safe to call as long as `xgrid_c` and `values_c` are arrays with sizes
/// larger or equal to `size`.
#[no_mangle]
pub unsafe extern "C" fn create_cubic_interpolator1d(
    xgrid_c: *const f64,
    values_c: *const f64,
    size: usize,
) -> Box<grid::cubic::Cubic1d> {
    // Use slice instead of vec, so that rust doesn't release the memory coming from C++
    let slice_input = unsafe { slice::from_raw_parts(xgrid_c, size) };
    // Make a copy of the data into a vector (of vectors) for rust to own
    let xgrid = vec![slice_input.to_vec()];
    let values = ArrayView1::from_shape_ptr(size, values_c);

    let grid = grid::Grid {
        xgrid,
        values: values.into_owned(),
    };
    let cubic_interpolator = grid::cubic::Cubic1d { grid };
    Box::new(cubic_interpolator)
}

/// Deletes an object created by [`create_cubic_interpolator1d`].
///
/// # Safety
///
/// The object given to this function must have been created by [`create_cubic_interpolator1d`] and
/// this function must not have been called with it before.
#[no_mangle]
pub unsafe extern "C" fn delete_cubic_interpolator1d(_: Box<grid::cubic::Cubic1d>) {}

/// Perform cubic1d interpolation in a previously generated interpolator
///
/// # Safety
///
/// The parameter `interpolator` must point to an object created by
/// [`create_cubic_interpolator1d`], otherwise this function is not safe to call.
///
/// # TODO
/// This doesn't need to be specific for 1D. Can I do it for any d?
#[no_mangle]
pub unsafe extern "C" fn interpolate_cubic_1d(
    interpolator: *mut grid::cubic::Cubic1d,
    query: f64,
) -> f64 {
    (*interpolator).interpolate(query).unwrap()
}
