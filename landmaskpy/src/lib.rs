use pyo3::prelude::*;
use numpy::PyArrayDyn;
use landmask::*;

#[pyclass(module = "landmask")]
struct Landmask {
    rtree: RTree<PolyWrapper>
}

#[pymethods]
impl Landmask {
    #[new]
    fn new (obj: &PyRawObject, path: String) {
        obj.init (Landmask {
            rtree: make_rtree_wkb(&path).unwrap()
        });
    }

    fn contains(&self, py: Python<'_>, x: f64, y: f64) -> PyResult<bool> {
        let c = py.allow_threads(move || contains (&self.rtree, x, y));
        Ok(c)
    }

    fn contains_many(&self,
        py: Python<'_>,
        x: &PyArrayDyn<f64>,
        y: &PyArrayDyn<f64>) -> Vec<bool> {
        let x = x.as_array();
        let y = y.as_array();

        let c = py.allow_threads(move ||
            contains_many(&self.rtree, x.as_slice().unwrap(), y.as_slice().unwrap()));

        c
    }

}

#[pymodule]
fn landmask(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Landmask>()?;

    Ok(())
}
