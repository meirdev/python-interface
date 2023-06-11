use pyo3::prelude::*;

#[pyfunction]
pub fn add(left: i64, right: i64) -> i64 {
    left + right
}

#[pymodule]
fn my_math(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;

    Ok(())
}
