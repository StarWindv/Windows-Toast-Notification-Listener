use crate::modules::types::{
    events::features::{callback_token::CallbackToken, events_type::EventsType},
    listener::Listener,
};

use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

use pyo3::{Py, PyAny, pyclass};

/// 事件循环主类，负责管理回调函数的注册、轮询调度与状态控制。
///
/// Attributes:
/// 
///     listener: 监听器实例，用于获取系统事件。
///     interval: 轮询间隔时间（单位：秒）。
///     registry: 回调函数注册表，存储所有注册的回调信息，包括回调对象、事件类型和是否可用。
///     running: 控制事件循环是否运行的原子布尔值。
///     interval_shared: 共享的轮询间隔值，支持动态修改。
#[pyclass(from_py_object)]
#[derive(Debug, Clone)]
pub struct Polling {
    pub listener: Listener,
    pub interval: i32,
    pub registry: Arc<Mutex<HashMap<CallbackToken, (Py<PyAny>, EventsType, bool)>>>,
    pub running: Arc<AtomicBool>,
    pub interval_shared: Arc<Mutex<i32>>,
}