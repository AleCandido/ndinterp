//! C interface for ndinterp
#![warn(clippy::all, clippy::cargo)]
#![warn(missing_docs)]

use core::slice;

use ndarray::ArrayView1;
use ndinterp::grid;
use ndinterp::interpolate::Interpolator;

/// Cubic1d interpolator
pub struct Cubic1d(grid::cubic::Cubic<1>);
/// Cubic2d interpolator
pub struct Cubic2d(grid::cubic::Cubic<2>);

/// Creates a Cubic1d interpolator given the nodes
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
) -> Box<Cubic1d> {
    // Use slice instead of vec, so that rust doesn't release the memory coming from C++
    let slice_input = unsafe { slice::from_raw_parts(xgrid_c, size) };
    // Make a copy of the data into a vector (of vectors) for rust to own
    let xgrid = vec![slice_input.to_vec()];
    let values = ArrayView1::from_shape_ptr(size, values_c);

    let grid = grid::Grid {
        xgrid,
        values: values.into_owned(),
    };
    let cubic_interpolator = Cubic1d(grid::cubic::Cubic { grid });
    Box::new(cubic_interpolator)
}

/// Deletes an object created by [`create_cubic_interpolator1d`].
///
/// # Safety
///
/// The object given to this function must have been created by [`create_cubic_interpolator1d`] and
/// this function must not have been called with it before.
#[no_mangle]
pub unsafe extern "C" fn delete_cubic_interpolator1d(_: Box<Cubic1d>) {}

/// Perform Cubic1d interpolation in a previously generated interpolator
///
/// # Safety
///
/// The parameter `interpolator` must point to an object created by
/// [`create_cubic_interpolator1d`], otherwise this function is not safe to call.
#[no_mangle]
pub unsafe extern "C" fn interpolate_cubic_1d(interpolator: *mut Cubic1d, query: f64) -> f64 {
    (*interpolator).0.interpolate(query).unwrap()
}

// 2D version of the functions above
/// Creates a cubic interpolator 2d Cubic2d
/// # Safety
///
/// This function is only safe to call as long as `xN_c` have size equal to `sizeN`
/// and the size of `values_c` is equal to size1*size2
#[no_mangle]
pub unsafe extern "C" fn create_cubic_interpolator2d(
    x1_c: *const f64,
    x2_c: *const f64,
    values_c: *const f64,
    size1: usize,
    size2: usize,
) -> Box<Cubic2d> {
    // Use slice instead of vec, so that rust doesn't release the memory coming from C++
    let slice_x1 = unsafe { slice::from_raw_parts(x1_c, size1) };
    let slice_x2 = unsafe { slice::from_raw_parts(x2_c, size2) };
    // Make a copy of the data into a vector (of vectors) for rust to own
    let xgrid = vec![slice_x1.to_vec(), slice_x2.to_vec()];
    let values = ArrayView1::from_shape_ptr(size1 * size2, values_c)
        .into_shape((size1, size2))
        .unwrap();

    let grid = grid::Grid {
        xgrid,
        values: values.into_owned(),
    };
    let cubic_interpolator = Cubic2d(grid::cubic::Cubic { grid });
    Box::new(cubic_interpolator)
}

/// Interpolate 2D
///
/// # Safety
///
/// The parameter `interpolator` must point to an object created by
/// [`create_cubic_interpolator2d`], otherwise this function is not safe to call.
#[no_mangle]
pub unsafe extern "C" fn interpolate_cubic_2d(interpolator: *mut Cubic2d, x1: f64, x2: f64) -> f64 {
    (*interpolator).0.interpolate(&[x1, x2]).unwrap()
}

/// Destructor 2D
///
/// # Safety
///
/// The object given to this function must have been created by [`create_cubic_interpolator2d`] and
/// this function must not have been called with it before.
#[no_mangle]
pub unsafe extern "C" fn delete_cubic_interpolator2d(_: Box<Cubic2d>) {}
