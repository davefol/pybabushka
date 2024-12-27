mod point2d;
mod polygon;

use point2d::Point2D;
use pyo3::prelude::*;

/// Python wrapper for babushka 
#[pymodule]
fn pybabushka(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Point2D>()?;
    Ok(())
}
