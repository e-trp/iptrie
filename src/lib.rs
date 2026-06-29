use pyo3::prelude::*;

pub mod ip;
pub mod py;

#[pymodule]
fn iptrie(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<py::PyTrie>()?;
    Ok(())
}
