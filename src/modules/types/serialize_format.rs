use pyo3::pyclass;

/// 用于`DiffTool.serialize_to`接口进行类型指示
#[pyclass(from_py_object, get_all)]
#[derive(Copy, Clone, PartialEq)]
pub enum SerializeFormat {
    Json,
    Yaml,
}
