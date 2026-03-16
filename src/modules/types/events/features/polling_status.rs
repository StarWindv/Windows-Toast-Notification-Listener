use pyo3::pyclass;

/// 此枚举类统一代表事件循环中各函数返回的状态。
///
/// Attributes:
/// 
///     Success: 操作成功完成
///     Failed: 操作失败
#[pyclass(from_py_object)]
#[derive(Debug, Clone, PartialEq)]
pub enum PollingStatus {
    Success,
    Failed,
}