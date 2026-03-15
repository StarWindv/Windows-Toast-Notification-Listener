use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

type PyResult<T> = Result<T, PyErr>;

pub trait ConvertToPyErr<T> {
    fn auto(self) -> PyResult<T>;
}

impl<T, E: std::fmt::Display> ConvertToPyErr<T> for Result<T, E> {
    fn auto(self) -> PyResult<T> {
        self.map_err(|e| PyRuntimeError::new_err(format!("[WNL Error] {}", e)))
    }
}
