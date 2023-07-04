use ndarray::ArrayView1;
/// C interface for ndinterp
use ndinterp::grid;
use ndinterp::interpolate::Interpolator;

pub struct Cubic1d;

/// Creates a cubic1d interpolator given the nodes
/// and the values of the function in said nodes
#[no_mangle]
pub unsafe extern "C" fn create_cubic_interpolator1d(
    input_c: *const f64,
    values_c: *const f64,
    size: usize,
) -> Box<grid::cubic::Cubic1d> {
    let input = unsafe { ArrayView1::from_shape_ptr((size,), input_c) };
    let values = unsafe { ArrayView1::from_shape_ptr((size,), values_c) };

    let grid = grid::Grid {
        input: input.into_owned(),
        values: values.into_owned(),
    };
    let cubic_interpolator = grid::cubic::Cubic1d { grid };
    return Box::new(cubic_interpolator);
}

/// Perform cubic1d interpolation in a previously generated interpolator
///
/// # TODO
/// This doesn't need to be specific for 1D. Can I do it for any d?
#[no_mangle]
pub unsafe extern "C" fn interpolate_cubic_1d(
    interpolator: *mut grid::cubic::Cubic1d,
    query: f64,
) -> f64 {
    let interpolator = &mut *interpolator;
    return interpolator.interpolate(query).unwrap();
}
