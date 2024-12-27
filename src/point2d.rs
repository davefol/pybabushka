use pyo3::prelude::*;
use babushka::point::Point2D as _;

#[pyclass]
#[derive(Clone, Copy)]
pub struct Point2D {
    pub inner: babushka::kernelf64::Point2D
}

#[pymethods]
impl Point2D {

    #[new]
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            inner: babushka::kernelf64::Point2D {x, y}
        }
    }

    pub fn __repr__(&self) -> String {
        format!("Point2D({}, {})", self.inner.x, self.inner.y).to_string()
    }

    pub fn x(&self) -> f64 {
        self.inner.x()
    }

    pub fn y(&self) -> f64 {
        self.inner.y()
    }
}