mod graph;

use pyo3::prelude::*;

#[pymodule]
fn networkg(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<graph::PyGraph>()?;

    Ok(())
}
