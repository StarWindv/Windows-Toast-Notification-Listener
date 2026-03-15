# About Event Notification Interface

This document explains why an API for event notification and callback registration was not included.

## Table of Contents

- [About Event Notification Interface](#about-event-notification-interface)
    - [Table of Contents](#table-of-contents)
    - [Explanation](#explanation)
        - [1.1 Historical Attempt](#11-historical-attempt)
        - [1.2 Limitations](#12-limitations)
        - [1.3 Other Attempts](#13-other-attempts)
        - [1.4 Outcome](#14-outcome)

---

## Explanation

### 1.1 Historical Attempt

In fact, we really did try to add an event notification interface. This work was originally scheduled for completion in version `0.0.3`.

Here was the draft code at that time:

```rust
use std::sync::Arc;

use pyo3::exceptions::PyTypeError;
use pyo3::types::PyAnyMethods;
use pyo3::{Bound, IntoPyObjectExt, PyAny, PyErr, Python};

use windows::core::{Ref as WinRef, Result as WinResult};
use windows::Foundation::TypedEventHandler;
use windows::UI::Notifications::{
    UserNotificationChangedEventArgs,
    UserNotificationChangedKind,
};

#[pymethods]
impl Listener {
    pub fn register_toast_handler(&self, handler: Bound<'_, PyAny>) -> PyResult<i64> {
        if !handler.is_callable() {
            return Err(PyErr::new::<PyTypeError, _>("handler must be callable"));
        }
        let listener_clone: UserNotificationListener = self.listener.clone();
        let handler = Arc::new(handler.unbind());
        let typed_handler =
            TypedEventHandler::<UserNotificationListener, UserNotificationChangedEventArgs>::new(
                move |_: WinRef<UserNotificationListener>,
                      args: WinRef<UserNotificationChangedEventArgs>|
                      -> WinResult<()> {
                    let change_kind = args.unwrap().ChangeKind()?;
                    let id = args.unwrap().UserNotificationId()?;

                    let handler = Arc::clone(&handler);
                    let listener_clone = listener_clone.clone();

                    let _ = Python::try_attach(|py| {
                        let handler = handler.bind(py);

                        let event_type: &str = match change_kind {
                            UserNotificationChangedKind::Added => "added",
                            UserNotificationChangedKind::Removed => "removed",
                            _ => return,
                        };

                        let data_result =
                            if matches!(change_kind, UserNotificationChangedKind::Added) {
                                if let Ok(raw_notif) = listener_clone.GetNotification(id) {
                                    if let Ok(parsed) = parse_notification(&raw_notif) {
                                        parsed.into_py_any(py)
                                    } else {
                                        Ok(py.None())
                                    }
                                } else {
                                    Ok(py.None())
                                }
                            } else {
                                id.into_py_any(py)
                            };

                        let data = data_result.unwrap_or_else(|_| py.None());

                        let _ = handler.call1((event_type, data));
                    });

                    Ok(())
                },
            );

        let token = self.listener.NotificationChanged(&typed_handler).auto()?;

        Ok(token)
    }

    pub fn unregister_toast_handler(&self, token: i64) -> PyResult<()> {
        self.listener.RemoveNotificationChanged(token).auto()
    }
}
```

This version of the code compiled successfully.

---

### 1.2 Limitations

Q: Since the code was already written, why wasn't it merged into the project?  
A: Because during testing, we encountered the following issue:

```shell
PS C:\> python -m asyncio
asyncio REPL 3.13.9 (tags/v3.13.9:8183fa5, Oct 14 2025, 14:09:13) [MSC v.1944 64 bit (AMD64)] on win32
Use "await" directly instead of "asyncio.run()".
Type "help", "copyright", "credits" or "license" for more information.
>>> import asyncio
>>> import win_notice_lite as wnl
>>> listener = wnl.Listener()
>>> await listener.request_permission()
'Allowed'
>>> listener.register_toast_handler(lambda *_, **__: None)
Traceback (most recent call last):
  File "[PythonRoot]\Lib\concurrent\futures\_base.py", line 449, in result
    return self.__get_result()
           ~~~~~~~~~~~~~~~~~^^
  File "[PythonRoot]\Lib\concurrent\futures\_base.py", line 401, in __get_result
    raise self._exception
  File "[PythonRoot]\Lib\asyncio\__main__.py", line 42, in callback
    coro = func()
  File "<python-input-4>", line 1, in <module>
    listener.register_toast_handler(lambda *_, **__: None)
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
RuntimeError: [WNL Error] The element was not found. (0x80070490)
```

It errored! But why? At the time, nobody knew. We scoured the `windows crate` inside and out, suspecting our own code might be wrong, but ultimately found nothing.

Later, after extensive research, we learned that, without exception, this interface behaves this way when used in **any non-C# packaged environment**.

[Here is another developer encountering a related issue](https://github.com/microsoft/WindowsAppSDK/issues/6172#issuecomment-3852349695)

---

### 1.3 Other Attempts

We did consider using the fallback solution mentioned in the comments of that issue, but it was too complicated, and there were very few existing examples available, so we abandoned the idea.

---

### 1.4 Outcome

The above summarizes why this project does not include event notification. We can only hope that Microsoft changes its mind and fixes this bug, whether intentional or not. When that happens, the code can be merged directly.
