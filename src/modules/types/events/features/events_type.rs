use pyo3::pyclass;

/// 用于表示回调函数需要处理的事件类型。
///
/// Attributes:
/// 
///     New: 表示新事件到达
///     Remove: 表示移除事件
///     All: 表示所有类型的事件（新事件 + 移除事件）
#[pyclass(from_py_object, get_all)]
#[derive(Debug, Clone, PartialEq)]
pub enum EventsType {
    New,
    Remove,
    All,
}