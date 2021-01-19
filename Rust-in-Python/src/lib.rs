use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn square(value: i128) -> PyResult<i128> {
    Ok(value.pow(2))
}

#[pymodule]
fn libsquare(_py: Python, module: &PyModule) -> PyResult<()> {
	module.add_function(wrap_pyfunction!(square, module)?)?;
	
    Ok(())
}