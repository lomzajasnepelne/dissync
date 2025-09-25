use pyo3::prelude::*;

#[pyfunction]
fn filter(a: Vec<u32>) -> PyResult<Vec<u32>> {
    let mut b = vec![0; a.len()];
    dissync_kalman::filter(a.as_slice(), b.as_mut_slice());
    Ok(b)
}

#[pymodule]
fn dissync_kalman_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(filter, m)?)?;
    Ok(())
}
