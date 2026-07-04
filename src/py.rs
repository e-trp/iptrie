use crate::ip::{Cidr, CidrTrie};
use pyo3::prelude::*;

#[pyclass]
pub struct PyTrie {
    inner: CidrTrie<Cidr<u32>>,
}

#[pymethods]
impl PyTrie {
    #[new]
    fn new() -> Self {
        Self {
            inner: CidrTrie::new(),
        }
    }

    fn insert(&mut self, cidr: &str) -> PyResult<()> {
        let parsed: Cidr<u32> = cidr
            .parse()
            .map_err(pyo3::exceptions::PyValueError::new_err)?;

        self.inner.insert(parsed);
        Ok(())
    }

    fn search(&self, cidr: &str) -> Option<String> {
        let parsed: Cidr<u32> = cidr.parse().ok()?;
        self.inner.search(&parsed).map(|v| v.to_string())
    }
}
