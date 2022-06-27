use pyo3::prelude::*;

/// PyO3 Python module that contains all exposed classes from Rust.
///
/// NOTE: this name has to match the one in Cargo.toml 'lib.name'
#[pymodule]
fn ndinterp(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("version", env!("CARGO_PKG_VERSION"))?;

    Ok(())
}
