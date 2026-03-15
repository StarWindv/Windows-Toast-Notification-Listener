# 关于事件通知接口

此文档用于解释为什么没有加入事件通知并注册回调函数的 API

## 目录

- [关于事件通知接口](#关于事件通知接口)
  - [目录](#目录)
  - [说明](#说明)
    - [1.1 历史尝试](#11-历史尝试)
    - [1.2 限制](#12-限制)
    - [1.3 其它尝试](#13-其它尝试)
    - [1.4 结果](#14-结果)

---

## 说明

### 1.1 历史尝试

事实上, 我们真的尝试过加入事件通知接口, 这项工作本应该在`0.0.3`时做完

这里是当时的初稿代码:

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

这个版本的代码是完全可以编译通过的

---

### 1.2 限制

Q: 既然连代码都写好了, 那么为什么没有合并入项目呢?
A: 因为我们在测试时, 遇到了以下问题:

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
RuntimeError: [WNL Error] 找不到元素。 (0x80070490)
```

它报错了! 但是为什么呢? 当时没有人知道, 我们把`windows crate`翻了个底朝天, 怀疑是不是自己代码写错了, 但终究一无所获

后来我们查询了很多资料才知道, 无一例外的, 此接口在**任何非 C# 打包环境**中使用时均会如此

[这里也有其他开发者遇到相关问题](https://github.com/microsoft/WindowsAppSDK/issues/6172#issuecomment-3852349695)

---

### 1.3 其它尝试

我们不是没想过用前面那个 issue 评论里的方案来做降级处理, 但是那太麻烦了, 而且现成示例少之又少, 故放弃.

---

### 1.4 结果

综上即为本项目没有加入事件通知的原因, 我们只能期待微软回心转意, 修好这个不知道是不是故意为之的`bug`, 那时就可以直接合并代码了.
