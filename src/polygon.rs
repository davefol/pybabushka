use pyo3::{prelude::*, types::PySequence};
use babushka::polygon::Polygon as _;

use crate::point2d::Point2D;

#[pyclass]
pub struct Polygon {
    inner: babushka::kernelf64::Polygon
}

#[pymethods]
impl Polygon {
    #[new]
    pub fn new(vertices: &Bound<'_, PyAny>, offset: Point2D, rotation: f64) -> PyResult<Self> {
        let vertices = vertices.downcast::<PySequence>()?;
        // let vertices: PySequence = vertices.downcast_bound(py)?;
        let mut _vertices = Vec::new();

        for item in vertices.try_iter()? {
            let item = item?;
            let point = item.extract::<Point2D>()?;
            _vertices.push(point.inner);
        }

        Ok(Self {
            inner: babushka::kernelf64::Polygon {
                vertices: _vertices,
                offset: offset.inner,
                rotation
            }
        })
    }
}