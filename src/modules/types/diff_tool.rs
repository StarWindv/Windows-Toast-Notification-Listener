use pyo3::pyclass;

/// 通知列表差异计算工具类
#[pyclass(from_py_object, get_all)]
#[derive(Debug, Clone)]
pub struct DiffTool {}
