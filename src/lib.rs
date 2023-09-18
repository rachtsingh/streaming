use pyo3::prelude::*;
use streaming_algorithms::CountMinSketch;

#[pyclass(name="CountMinSketch")]
struct PyCountMinSketch {
    inner: CountMinSketch<u64, u64>,
}

#[pymethods]
impl PyCountMinSketch {
    #[new]
    fn new(probability: f64, tolerance: f64) -> Self {
        PyCountMinSketch {
            inner: CountMinSketch::new(probability, tolerance, ()),
        }
    }

    fn add(&mut self, key: u64, value: u64) {
        self.inner.push(&key, &value);
    }

    fn get(&self, key: u64) -> u64 {
        self.inner.get(&key)
    }

    fn clear(&mut self) {
        self.inner.clear();
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn streaming(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyCountMinSketch>()?;
    Ok(())
}