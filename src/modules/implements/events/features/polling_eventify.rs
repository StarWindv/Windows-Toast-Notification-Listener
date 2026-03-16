use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

use crate::modules::types::{
    diff_tool::DiffTool, events::features::callback_token::CallbackToken,
    events::features::events_type::EventsType, events::features::polling_eventify::Polling,
    events::features::polling_status::PollingStatus, listener::Listener, toast::Toast,
};

use pyo3::{
    Bound, Py, PyAny, PyErr, PyResult, Python, exceptions::PyTypeError, pymethods,
    types::PyAnyMethods,
};

use crate::modules::error::ConvertToPyErr;
use tokio::{
    runtime::Runtime,
    time::{Duration, sleep},
};

static RUNTIME: OnceLock<Runtime> = OnceLock::new();

#[pymethods]
impl Polling {
    /// 创建一个新的 Polling 实例。
    ///
    /// Args:
    ///     listener (Listener): 用于获取通知的监听器实例。
    ///     interval (int): 轮询间隔时间（毫秒）。
    ///
    /// Returns:
    ///     Polling: 返回一个新的 Polling 对象。
    #[new]
    pub fn new(listener: Listener, interval: i32) -> PyResult<Self> {
        Ok(Self {
            listener,
            interval,
            registry: Arc::new(Mutex::new(HashMap::new())),
            running: Arc::new(AtomicBool::new(false)),
            interval_shared: Arc::new(Mutex::new(interval)),
        })
    }

    /// 注册一个全局轮询事件回调函数，该回调会接收所有类型的事件。
    ///
    /// Args:
    ///     handler (Callable): 一个可调用对象，接收一个 Diff 参数。
    ///
    /// Returns:
    ///     CallbackToken: 用于后续取消注册的令牌。
    ///
    /// Raises:
    ///     TypeError: 如果 handler 不是可调用的函数
    pub fn register_polling_event_callback(
        &self,
        handler: Bound<'_, PyAny>,
    ) -> PyResult<CallbackToken> {
        if !handler.is_callable() {
            return Err(PyErr::new::<PyTypeError, _>(
                "[WNL Error] Handler must be callable",
            ));
        }
        let token = CallbackToken::new();
        let py_handler = handler.unbind();
        let mut reg = self.registry.lock().auto()?;
        reg.insert(token.clone(), (py_handler, EventsType::All, true));
        Ok(token)
    }

    /// 注销指定令牌对应的回调函数。
    ///
    /// Args:
    ///
    ///     token (CallbackToken): 要取消注册的回调令牌。
    ///
    /// Returns:
    ///
    ///     PollingStatus: 返回 Success 如果成功移除，否则返回 Failed。
    pub fn unregister(&self, token: CallbackToken) -> PyResult<PollingStatus> {
        let mut reg = self.registry.lock().auto()?;
        if reg.remove(&token).is_some() {
            Ok(PollingStatus::Success)
        } else {
            Ok(PollingStatus::Failed)
        }
    }

    /// 注册一个仅针对特定事件类型的回调函数。
    ///
    /// Args:
    ///     handler (Callable): 接收 Diff 参数的可调用对象。
    ///     for_type (EventsType): 指定该回调只响应的通知类型
    ///
    /// Returns:
    ///     CallbackToken: 用于后续取消注册的令牌。
    ///
    /// Raises:
    ///     TypeError: 如果 handler 不是可调用的。
    pub fn on_type_callback(
        &self,
        handler: Bound<'_, PyAny>,
        for_type: EventsType,
    ) -> PyResult<CallbackToken> {
        if !handler.is_callable() {
            return Err(PyErr::new::<PyTypeError, _>(
                "[WNL Error] Handler must be callable",
            ));
        }
        let token = CallbackToken::new();
        let py_handler = handler.unbind();
        let mut reg = self.registry.lock().auto()?;
        reg.insert(token.clone(), (py_handler, for_type, true));
        Ok(token)
    }

    /// 启动事件循环, 并将可能的回调函数投入任务中
    ///
    /// 如果轮询已经在运行，则立即返回 Success。
    ///
    /// Returns:
    ///
    ///     PollingStatus: 返回 Success。
    pub fn start_all(&self) -> PyResult<PollingStatus> {
        if self.running.load(Ordering::SeqCst) {
            return Ok(PollingStatus::Success);
        }
        self.running.store(true, Ordering::SeqCst);

        let listener = self.listener.clone();
        let registry = self.registry.clone();
        let running = self.running.clone();
        let interval_shared = self.interval_shared.clone();

        let rt = RUNTIME.get_or_init(|| Runtime::new().expect("Failed to create tokio runtime"));
        rt.spawn(async move {
            Self::run_polling(listener, registry, running, interval_shared).await;
        });

        Ok(PollingStatus::Success)
    }

    /// 停止所有轮询任务。
    ///
    /// Returns:
    ///
    ///     PollingStatus: 返回 Success。
    pub fn stop_all(&self) -> PyResult<PollingStatus> {
        self.running.store(false, Ordering::SeqCst);
        Ok(PollingStatus::Success)
    }

    /// 激活指定令牌的回调函数，使其开始处理事件。
    ///
    /// Args:
    ///
    ///     token (CallbackToken): 要激活的回调令牌。
    ///
    /// Returns:
    ///
    ///     PollingStatus: 如果找到该令牌则返回 Success，否则返回 Failed。
    pub fn polling_for(&self, token: CallbackToken) -> PyResult<PollingStatus> {
        let mut reg = self.registry.lock().auto()?;
        if let Some((_, _, active)) = reg.get_mut(&token) {
            *active = true;
            Ok(PollingStatus::Success)
        } else {
            Ok(PollingStatus::Failed)
        }
    }

    /// 停止指定令牌的回调函数，使其不再处理事件。
    ///
    /// Args:
    ///     token (CallbackToken): 要暂停的回调令牌。
    ///
    /// Returns:
    ///     PollingStatus: 如果找到该令牌则返回 Success，否则返回 Failed。
    pub fn stop_for(&self, token: CallbackToken) -> PyResult<PollingStatus> {
        let mut reg = self.registry.lock().auto()?;
        if let Some((_, _, active)) = reg.get_mut(&token) {
            *active = false;
            Ok(PollingStatus::Success)
        } else {
            Ok(PollingStatus::Failed)
        }
    }

    /// 动态修改轮询间隔时间。
    ///
    /// Args:
    ///
    ///     interval (int): 新的轮询间隔时间（毫秒）。
    pub fn change_interval(&mut self, interval: i32) {
        self.interval = interval;
        *self.interval_shared.lock().unwrap() = interval;
    }
}

impl Polling {
    async fn run_polling(
        listener: Listener,
        registry: Arc<Mutex<HashMap<CallbackToken, (Py<PyAny>, EventsType, bool)>>>,
        running: Arc<AtomicBool>,
        interval_shared: Arc<Mutex<i32>>,
    ) {
        let mut previous: Vec<Toast> = vec![];
        while running.load(Ordering::SeqCst) {
            let current = match listener.get_all_notifications().await {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("[WNL Polling] get_all_notifications error: {:?}", e);
                    sleep(Duration::from_millis(1000)).await;
                    continue;
                }
            };

            let diff = DiffTool::diff_full(previous.clone(), current.clone());
            previous = current;

            if diff.new.is_empty() && diff.remove.is_empty() {
                let ms = *interval_shared.lock().unwrap() as u64;
                sleep(Duration::from_millis(ms)).await;
                continue;
            }

            let _ = Python::attach(|py| {
                let py_diff = match Py::new(py, diff.clone()) {
                    Ok(p) => p.into_bound(py).into_any(),
                    Err(_) => return,
                };

                let guard = registry.lock().unwrap();

                let handlers_to_call = guard
                    .iter()
                    .filter_map(|(_, (h, e_type, active))| {
                        if !*active {
                            return None;
                        }
                        match e_type {
                            EventsType::New if !diff.new.is_empty() => Some(h),
                            EventsType::Remove if !diff.remove.is_empty() => Some(h),
                            EventsType::All => Some(h),
                            _ => None,
                        }
                    })
                    .collect::<Vec<_>>();

                for handler in handlers_to_call {
                    let _ = handler.call1(py, (&py_diff,));
                }
            });

            let ms = *interval_shared.lock().unwrap() as u64;
            sleep(Duration::from_millis(ms)).await;
        }
    }
}
