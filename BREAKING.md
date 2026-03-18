# 破坏性更新与更新说明

此文件用于说明各版本中 Python 侧 API 的变动

---

## 目录
- [破坏性更新与更新说明](#破坏性更新与更新说明)
- [目录](#目录)
- [详细信息](#详细信息)
    - [v0.0.1](#v001)
    - [v0.0.1a0](#v001a0)
    - [v0.0.2](#v002)
    - [v0.0.3](#v003)
    - [v0.0.4](#v004)
    - [v0.1.0](#v010)
    - [v0.1.1](#v011)

---

## 详细信息

### v0.0.1
此版本是项目的初始版本, 不存在破坏性更新

---

### v0.0.1a0
此版本实际上是初始版本的补丁, 处理了 readme 中的部分问题, 只是版本号写错了

---

### v0.0.2
此版本仅做了 API 的位置迁移

| Old API                      | New API                    |
|------------------------------|----------------------------|
| `Listener.diff_full`         | `Differ.diff_full`         |
| `Listener.diff_without_time` | `Differ.diff_without_time` |
| `Listener.diff_by_id`        | `Differ.diff_by_id`        |
| `Listener.serialize`         | `Differ.serialize`         |

---

### v0.0.3
#### 已有接口变化
| 维度       | 内容                                                                                                  |
|----------|-----------------------------------------------------------------------------------------------------|
| API Name | `Listener.elevate_privilege` → `Listener.request_permission`                                        |
| Args     | `No Args` → `No Args`                                                                               |
| Returns  | `UserNotificationListenerAccessStatus(0/1/2)`, str → `Unspecified/Allowed/Denied/UnknownError`, str |

#### 名称迁移
| Old API                     | New API                                                 | Explanation |
|-----------------------------|---------------------------------------------------------|-------------|
| `Differ.diff_full`          | `DiffTool.diff_full`                                    | 名称迁移        |
| `Differ.diff_without_time`  | `DiffTool.diff_without_time`                            | 名称迁移        |
| `Differ.diff_by_id`         | `DiffTool.diff_by_id`                                   | 名称迁移        |
| `Differ.serialize([Toast])` | `DiffTool.serialize_to([Toast], Type: SerializeFormat)` | 参数改变、名称迁移   |
| -                           | `DiffTool.to_json_str([Toast])`                         | 新 API       |

#### 新的类

| 类名              | 类型  | 成员                    | 接口 | 是否可实例化 |
|-----------------|-----|-----------------------|----|--------|
| SerializeFormat | 枚举类 | Json, Yaml, Toml, XML | 无  | 否      |

---

### v0.0.4

#### 新 API
- `DiffTool.generate_fingerprint(Toast, include_time: bool)`: 为指定的单个通知生成指纹
- `Toast.from_dict`: 直接从 Python 字典中生成 Toast 对象
- `MutableToast.from_dict`: 直接从 Python 字典中生成 MutableToast 对象

#### 新的类
| 类名        | 类型  | 作用   | 接口 | 是否可实例化 |
|-----------|-----|------|----|--------|
| ToastDict | 普通类 | 类型注解 | 无  | 可以     |

> **注意**: Toast/MutableToast 的 `from_dict` 方法和 `ToastDict` 来自于`__init__.py`文件定义, 并不是 Rust 代码实现的

#### 已删除的 API
- `SerializeFormat.Toml`
- `SerializeFormat.XML`

**删除原因**:
- `Toast` 不适合被序列化为这两种格式, 并且实际序列化时一直在报错, 故删除

---

### v0.1.0

#### 新模块: 事件通知系统 (Event Notification System)

本次更新引入了完整的事件通知系统, 允许开发者以回调方式监听通知变化, 人工修复了 Windows 所未修复的 BUG 导致的 API 缺失

##### 新的类

| 类名                           | 类型  | 作用        | 接口                                           | 是否可实例化 |
|------------------------------|-----|-----------|----------------------------------------------|--------|
| `wnl.features.Polling`       | 普通类 | 事件循环管理器   | `start_all`, `stop_all`, `change_interval` 等 | 可以     |
| `wnl.features.CallbackToken` | 普通类 | 回调函数唯一标识符 | `id` 属性                                      | 可以     |
| `wnl.features.PollingStatus` | 枚举类 | 操作状态      | `Success`, `Failed`                          | 否      |
| `wnl.features.EventsType`    | 枚举类 | 事件类型      | `New`, `Remove`, `All`                       | 否      |

##### 新的 API

| API                                        | 所属类                    | 说明               |
|--------------------------------------------|------------------------|------------------|
| `register_polling_event_callback(handler)` | `wnl.features.Polling` | 注册全局回调, 接收所有类型事件 |
| `on_type_callback(handler, for_type)`      | `wnl.features.Polling` | 注册仅针对特定类型事件的回调   |
| `unregister(token)`                        | `wnl.features.Polling` | 注销指定令牌的回调        |
| `polling_for(token)`                       | `wnl.features.Polling` | 激活指定回调           |
| `stop_for(token)`                          | `wnl.features.Polling` | 暂停指定回调           |
| `start_all()`                              | `wnl.features.Polling` | 启动事件循环           |
| `stop_all()`                               | `wnl.features.Polling` | 停止事件循环           |
| `change_interval(ms)`                      | `wnl.features.Polling` | 动态修改轮询间隔         |

##### 使用示例

```python
import win_notice_lite as wnl

listener = wnl.Listener()
polling = wnl.features.Polling(listener, interval=1000)

def on_new_notification(diff):
    for toast in diff.new:
        print(f"新通知: {toast.title}")

token = polling.on_type_callback(on_new_notification, wnl.EventsType.New)
polling.start_all()  # 启动轮询

# 稍后...
polling.stop_for(token)      # 暂停此回调
polling.polling_for(token)   # 重新激活
polling.change_interval(2000) # 修改轮询间隔
polling.stop_all()            # 停止所有
```

---

### v0.1.1

#### 新的 API

##### 覆写默认 __str__/__repr__

###### 作用范围

`Toast`, `MutableTOast`, `Diff`, `CallbackToken`

###### 修改目的

使得输出更加直观

###### 新返回值示例

对于 Toast/MutableToast:
```json
{
  "id": 52161,
  "name": "Python",
  "logo_uri": "",
  "title": "My Custom Toast",
  "message": "This is: 1",
  "hero_image_uri": "",
  "inline_images": [],
  "tag": "",
  "group": "",
  "creation_time": "134182754669638508",
  "fingerprint": "a654709ca23b4ccbe16216e94a567b805a02eb72bdd1dafe03770ff5054fa7e1",
  "fingerprint_without_time": "092097705025e2bd0a7fbb27f9250271e128f6fa4e4770e8595c03648bf72eb8"
}
```

对于 Diff:
```json
{
  "new": [
  {
    "id": 52171,
    "name": "Python",
    "logo_uri": "",
    "title": "My Custom Toast",
    "message": "This is: 2",
    "hero_image_uri": "",
    "inline_images": [],
    "tag": "",
    "group": "",
    "creation_time": "134182764652809624",
    "fingerprint": "2fbd65b8c1ac18bc4a7d3f99512867dedb03c284ba8a747297adac2b3d4b84e1",
    "fingerprint_without_time": "b102f0db9f8b49f3915db59465448395d3cbcc8441dd140fb66a8fdb9c30fdb0"
  }
],
  "remove": []
}
```

对于 CallbackToken:
```plaintext
CallbackToken(number)
```

##### 新增方法: win_notice_lite.features.Polling.show_registry

###### 修改目的

使得输出更加直观

###### 返回值示例

```json
  {
    "CallbackToken(0)": {
      "handler": "Py(0x1d063df5440)",
      "events_type": "All",
      "is_active": true
    }
  }
```

---

末次编辑日期: 2026年3月18日
