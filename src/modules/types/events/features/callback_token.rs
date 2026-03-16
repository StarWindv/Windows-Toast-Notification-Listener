use pyo3::pyclass;
use std::sync::atomic::AtomicU64;

pub static TOKEN_COUNTER: AtomicU64 = AtomicU64::new(0);

/// 为每个回调函数生成唯一的标识符，并支持存入注册表。
///
/// Attributes:
///
///     id: 回调函数的唯一标识符，由系统自动生成。
#[pyclass(from_py_object, get_all)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CallbackToken {
    pub id: u64,
}