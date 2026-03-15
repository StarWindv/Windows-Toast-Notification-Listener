mod modules;
use modules::types;

use pyo3::prelude::{PyModule, PyModuleMethods};
use pyo3::{Bound, PyResult, Python, pymodule};

#[pymodule]
fn win_notice_lite(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<types::listener::Listener>()?;
    m.add_class::<types::toast::Toast>()?;
    m.add_class::<types::mutable_toast::MutableToast>()?;
    m.add_class::<types::diff::Diff>()?;
    m.add_class::<types::diff_tool::DiffTool>()?;
    m.add_class::<types::serialize_format::SerializeFormat>()?;
    Ok(())
}
