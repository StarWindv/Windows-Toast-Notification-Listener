use crate::modules::types::{
    diff::Diff,
    diff_tool::DiffTool,
    toast::Toast
};

use pyo3::{PyResult, pymethods};

#[pymethods]
impl Diff {
    #[new]
    pub fn __init__(new: Vec<Toast>, remove: Vec<Toast>) -> PyResult<Self> {
        Ok(Self { new, remove })
    }

    pub fn __str__(&self) -> String {
        let new = DiffTool::to_json_str(self.clone().new).unwrap();
        let remove = DiffTool::to_json_str(self.clone().remove).unwrap();
        format!(
            "{{\n  \"new\": {},\n  \"remove\": {}\n}}",
            new, remove
        )
    }

    pub fn __repr__(&self) -> String {
        self.__str__()
    }
}
