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

    fn insert(&mut self, cidr: &str) -> Option<bool> {
        let parsed: Cidr<u32> = cidr.parse().ok()?;
        self.inner.insert(parsed)
    }

    fn search(&self, cidr: &str) -> Option<String> {
        let parsed: Cidr<u32> = cidr.parse().ok()?;
        self.inner.search(&parsed).map(|v| v.to_string())
    }

    fn search_supernets(&self, cidr: &str) -> Option<Vec<String>> {
        let parsed: Cidr<u32> = cidr.parse().ok()?;
        self.inner.search_supernets(&parsed).map(|result| {
            result
                .into_iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
        })
    }

    fn search_subnets(&self, cidr: &str) -> Option<Vec<String>> {
        let parsed: Cidr<u32> = cidr.parse().ok()?;
        self.inner.search_subnets(&parsed).map(|result| {
            result
                .into_iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
        })
    }
}
