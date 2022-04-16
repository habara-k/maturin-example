// https://github.com/konstin/complex-manylinux-maturin-docker/blob/main/nightly-only/src/lib.rs

#![feature(box_syntax)] // Make this crate nightly only

use pyo3::prelude::*;

#[pyclass]
struct DummyClass {}

#[pymethods]
impl DummyClass {
    #[staticmethod]
    fn get_42() -> PyResult<usize> {
        Ok(42)
    }
}

#[pymodule]
fn maturin_example(_py: Python, m: &PyModule) -> PyResult<()> {
    let _five = box 5;

    m.add_class::<DummyClass>()?;
    m.add("fourtytwo", 42)?;

    Ok(())
}
